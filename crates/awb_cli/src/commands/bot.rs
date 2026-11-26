// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_bot::{BotConfig, BotRunner, Checkpoint};
use awb_domain::i18n::{Catalog, Locale, fmt1};
use awb_domain::rules::RuleSet;
use awb_engine::general_fixes::FixRegistry;
use awb_engine::transform::TransformEngine;
use awb_mw_api::client::MediaWikiClient;
use console::style;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use url::Url;

/// Arguments for the bot run command
pub struct BotRunArgs {
    pub wiki: Url,
    pub profile_path: PathBuf,
    pub max_edits: Option<u32>,
    pub dry_run: bool,
    pub checkpoint_path: Option<PathBuf>,
    pub auth_profile: String,
    pub skip_no_change: bool,
    pub skip_on_warning: bool,
    pub log_every_n: u32,
    pub locale: Locale,
}

fn mode_label(dry_run: bool, cat: &Catalog) -> console::StyledObject<&'static str> {
    if dry_run {
        style(cat.mode_dry_run).yellow()
    } else {
        style(cat.mode_autonomous).green().bold()
    }
}

fn load_or_create_checkpoint(checkpoint_path: Option<&Path>, cat: &Catalog) -> Result<Checkpoint> {
    if let Some(path) = checkpoint_path
        && path.exists()
    {
        println!(
            "{}",
            fmt1(
                cat.msg_loading_checkpoint,
                "path",
                &path.display().to_string()
            )
        );
        return Checkpoint::load(path).context(cat.err_load_checkpoint);
    }
    Ok(Checkpoint::new())
}

pub async fn run(args: BotRunArgs) -> Result<()> {
    let cat = args.locale.catalog();
    super::emit_stdout_line(&format!("{}", style(cat.bot_title).bold().cyan()));
    super::emit_stdout_line(&format!("{}: {}", cat.label_wiki, args.wiki));
    super::emit_stdout_line(&format!(
        "{}: {}",
        cat.label_profile,
        args.profile_path.display()
    ));
    super::emit_stdout_line(&format!(
        "{}: {}",
        cat.label_mode,
        mode_label(args.dry_run, cat)
    ));
    if let Some(max) = args.max_edits {
        super::emit_stdout_line(&format!("{}: {}", cat.label_max_edits, max));
    }
    super::emit_stdout_line("");

    let (client, password) = super::connect_authenticated_client(
        &args.wiki,
        &args.profile_path,
        &args.auth_profile,
        cat,
    )
    .await?;

    // For demo purposes, generate a simple page list
    // In real usage, this would come from the profile configuration or a list command
    let pages = vec![
        "Test Page 1".to_string(),
        "Test Page 2".to_string(),
        "Test Page 3".to_string(),
    ];

    let report_path = PathBuf::from(format!(
        "bot-report-{}.json",
        chrono::Utc::now().format("%Y%m%d-%H%M%S")
    ));

    execute_bot_run(client, password, &args, pages, &report_path, cat).await
}

async fn execute_bot_run<C: MediaWikiClient>(
    client: C,
    password: String,
    args: &BotRunArgs,
    pages: Vec<String>,
    report_path: &Path,
    cat: &Catalog,
) -> Result<()> {
    // Load rules and build engine
    let ruleset = RuleSet::new(); // In production, load from profile
    let registry = FixRegistry::with_defaults();
    let enabled_fixes = HashSet::new(); // In production, load from profile

    let engine =
        TransformEngine::new(&ruleset, registry, enabled_fixes).context(cat.err_create_engine)?;

    println!(
        "{}",
        fmt1(cat.msg_processing_pages, "count", &pages.len().to_string())
    );
    println!();

    // Configure bot
    let mut bot_config = BotConfig::new()
        .with_skip_no_change(args.skip_no_change)
        .with_skip_on_warning(args.skip_on_warning)
        .with_log_every_n(args.log_every_n)
        .with_dry_run(args.dry_run);

    if let Some(max) = args.max_edits {
        bot_config = bot_config.with_max_edits(max);
    }

    // Load or create checkpoint
    let checkpoint = load_or_create_checkpoint(args.checkpoint_path.as_deref(), cat)?;

    // Create and run bot
    let mut bot_runner = if checkpoint.next_index() > 0 {
        println!(
            "{}",
            fmt1(
                cat.msg_resuming_checkpoint,
                "page",
                &(checkpoint.next_index() + 1).to_string()
            )
        );
        BotRunner::with_checkpoint(bot_config, client, engine, pages, checkpoint)
    } else {
        BotRunner::new(bot_config, client, engine, pages)
    };

    // Register secrets for redaction in error messages
    bot_runner.add_secret(password);

    let report = match bot_runner.run().await {
        Ok(report) => report,
        Err(e) => {
            eprintln!(
                "{} {}",
                style("✗").red(),
                fmt1(cat.msg_bot_error, "error", &e.to_string())
            );

            // Save checkpoint on error
            if let Some(ref path) = args.checkpoint_path {
                if let Err(e) = bot_runner.save_checkpoint(path) {
                    eprintln!(
                        "{} {}",
                        style("✗").red(),
                        fmt1(cat.msg_failed_save_checkpoint, "error", &e.to_string())
                    );
                } else {
                    println!("{} {}", style("ℹ").cyan(), cat.msg_checkpoint_saved);
                }
            }

            return Err(e.into());
        }
    };

    // Save final checkpoint
    if let Some(ref path) = args.checkpoint_path {
        bot_runner
            .save_checkpoint(path)
            .context(cat.err_save_final_checkpoint)?;
    }

    // Display report
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", report.to_summary_localized(args.locale));
    println!("{}", style("═".repeat(60)).dim());

    // Save JSON report
    tokio::fs::write(report_path, report.to_json()?)
        .await
        .context(cat.err_save_report)?;
    println!(
        "{}",
        fmt1(
            cat.msg_report_saved,
            "path",
            &report_path.display().to_string()
        )
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_support::{TestTempDir, write_test_profile};

    #[test]
    fn test_bot_mode_label() {
        for locale in [Locale::En, Locale::De, Locale::Ja] {
            let cat = locale.catalog();
            assert!(mode_label(true, cat).to_string().contains(cat.mode_dry_run));
            assert!(
                mode_label(false, cat)
                    .to_string()
                    .contains(cat.mode_autonomous)
            );
        }
    }

    #[test]
    fn test_load_or_create_checkpoint() {
        let tmp = TestTempDir::new("bot-ckpt");
        let cat = Locale::De.catalog();

        let fresh = load_or_create_checkpoint(None, cat).expect("none yields new checkpoint");
        assert_eq!(fresh.next_index(), 0);

        let missing = tmp.path().join("missing.json");
        let fresh_missing = load_or_create_checkpoint(Some(&missing), cat)
            .expect("nonexistent path yields new checkpoint");
        assert_eq!(fresh_missing.next_index(), 0);

        let saved_path = tmp.path().join("saved.json");
        let mut ckpt = Checkpoint::new();
        ckpt.record_page("Page A".to_string(), true, false, false);
        let serialized = serde_json::to_string_pretty(&ckpt).expect("serialize checkpoint");
        std::fs::write(&saved_path, serialized).expect("write checkpoint");
        let loaded =
            load_or_create_checkpoint(Some(&saved_path), cat).expect("load saved checkpoint");
        assert_eq!(loaded.next_index(), 1);
        assert!(loaded.is_completed("Page A"));

        let corrupt_path = tmp.path().join("corrupt.json");
        std::fs::write(&corrupt_path, b"{not valid json").expect("write corrupt checkpoint");
        let err = load_or_create_checkpoint(Some(&corrupt_path), cat).unwrap_err();
        assert!(err.to_string().contains(cat.err_load_checkpoint));
    }

    #[tokio::test]
    async fn test_bot_run_missing_profile_and_missing_creds() {
        let tmp = TestTempDir::new("bot-run");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();

        let missing = tmp.path().join("missing.toml");
        let err = run(BotRunArgs {
            wiki: wiki.clone(),
            profile_path: missing,
            max_edits: Some(5),
            dry_run: true,
            checkpoint_path: None,
            auth_profile: "default".to_string(),
            skip_no_change: true,
            skip_on_warning: false,
            log_every_n: 10,
            locale: Locale::De,
        })
        .await
        .unwrap_err();
        assert!(
            err.to_string()
                .contains(Locale::De.catalog().err_load_profile)
        );

        let valid = tmp.path().join("profiles.toml");
        write_test_profile(&valid, "bot-main", &wiki);
        let err = run(BotRunArgs {
            wiki,
            profile_path: valid,
            max_edits: None,
            dry_run: false,
            checkpoint_path: None,
            auth_profile: "bot-main".to_string(),
            skip_no_change: true,
            skip_on_warning: true,
            log_every_n: 5,
            locale: Locale::Fr,
        })
        .await
        .unwrap_err();
        assert!(
            err.to_string()
                .contains(Locale::Fr.catalog().err_no_stored_creds)
        );
    }

    #[tokio::test]
    #[cfg_attr(
        windows,
        ignore = "Checkpoint::save opens parent directory which fails on Windows"
    )]
    async fn test_execute_bot_run_fresh_and_resumed() {
        use crate::commands::test_support::MockMwClient;

        let tmp = TestTempDir::new("bot-exec");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();
        let ckpt_path = tmp.path().join("checkpoint.json");
        let report_path = tmp.path().join("report.json");

        let mut client = MockMwClient::new();
        client.add_page("Page 1", "Clean text");
        client.add_page("Page 2", "More text");

        let args = BotRunArgs {
            wiki: wiki.clone(),
            profile_path: tmp.path().join("profiles.toml"),
            max_edits: Some(5),
            dry_run: true,
            checkpoint_path: Some(ckpt_path.clone()),
            auth_profile: "default".to_string(),
            skip_no_change: true,
            skip_on_warning: false,
            log_every_n: 1,
            locale: Locale::De,
        };

        execute_bot_run(
            client,
            crate::commands::test_support::dummy_credential(),
            &args,
            vec!["Page 1".to_string(), "Page 2".to_string()],
            &report_path,
            Locale::De.catalog(),
        )
        .await
        .expect("fresh bot run succeeds");
        assert!(ckpt_path.is_file());
        assert!(report_path.is_file());

        // Second run resumes from the saved checkpoint (next_index > 0)
        let mut client2 = MockMwClient::new();
        client2.add_page("Page 1", "Clean text");
        client2.add_page("Page 2", "More text");
        let report_path2 = tmp.path().join("report2.json");
        execute_bot_run(
            client2,
            crate::commands::test_support::dummy_credential(),
            &args,
            vec!["Page 1".to_string(), "Page 2".to_string()],
            &report_path2,
            Locale::Ja.catalog(),
        )
        .await
        .expect("resumed bot run succeeds");
        assert!(report_path2.is_file());
    }
}
