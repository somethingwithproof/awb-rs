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
