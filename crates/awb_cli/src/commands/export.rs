// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Locale, fmt2};
use awb_telemetry::{ExportFormat as TelemetryFormat, export_log};
use console::style;
use std::path::PathBuf;

use crate::ExportFormat;

pub async fn run(format: ExportFormat, output: PathBuf, locale: Locale) -> Result<()> {
    let cat = locale.catalog();
    println!("{}", style(cat.export_title).bold().cyan());
    println!("{}: {:?}", cat.label_format, format);
    println!("{}: {}", cat.label_output, output.display());
    println!();

    // For now, we'll use an empty event list as a placeholder
    // In a real implementation, this would load events from a telemetry store
    let events = vec![];

    let telemetry_format = match format {
        ExportFormat::Json => TelemetryFormat::Json,
        ExportFormat::Csv => TelemetryFormat::Csv,
        ExportFormat::Plain => TelemetryFormat::PlainText,
    };

    let mut file = tokio::fs::File::create(&output)
        .await
        .context(cat.err_create_output_file)?
        .into_std()
        .await;

    export_log(&events, telemetry_format, &mut file).context(cat.err_export_log)?;

    println!(
        "{} {}",
        style("✓").green().bold(),
        fmt2(
            cat.msg_exported_events,
            "count",
            &events.len().to_string(),
            "path",
            &output.display().to_string()
        )
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_support::TestTempDir;

    #[tokio::test]
    async fn test_export_run_formats_and_error() {
        let tmp = TestTempDir::new("export");

        let json_out = tmp.path().join("telemetry.json");
        run(ExportFormat::Json, json_out.clone(), Locale::En)
            .await
            .expect("json export succeeds");
        let json_content = std::fs::read_to_string(&json_out).expect("read json output");
        assert!(json_content.is_empty());

        let csv_out = tmp.path().join("telemetry.csv");
        run(ExportFormat::Csv, csv_out.clone(), Locale::De)
            .await
            .expect("csv export succeeds");
        let csv_content = std::fs::read_to_string(&csv_out).expect("read csv output");
        assert!(csv_content.starts_with("type,timestamp,details"));

        let plain_out = tmp.path().join("telemetry.txt");
        run(ExportFormat::Plain, plain_out.clone(), Locale::Ja)
            .await
            .expect("plain export succeeds");
        assert!(plain_out.is_file());

        let invalid_out = tmp.path().join("missing_parent_dir").join("out.json");
        let err = run(ExportFormat::Json, invalid_out, Locale::Fr)
            .await
            .unwrap_err();
        assert!(
            err.to_string()
                .contains(Locale::Fr.catalog().err_create_output_file)
        );
    }
}
