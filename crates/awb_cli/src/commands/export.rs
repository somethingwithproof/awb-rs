use anyhow::{Context, Result};
use awb_telemetry::{ExportFormat as TelemetryFormat, export_log};
use console::style;
use std::path::PathBuf;

use crate::ExportFormat;

pub async fn run(format: ExportFormat, output: PathBuf) -> Result<()> {
    println!("{}", style("Export Telemetry Log").bold().cyan());
    println!("Format: {:?}", format);
    println!("Output: {}", output.display());
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
        .context("Failed to create output file")?
        .into_std()
        .await;

    export_log(&events, telemetry_format, &mut file).context("Failed to export log")?;

    println!(
        "{} Exported {} events to {}",
        style("✓").green().bold(),
        events.len(),
        output.display()
    );

    Ok(())
}
