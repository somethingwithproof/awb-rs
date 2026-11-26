// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_bot::{BotConfig, BotRunner, Checkpoint};
use awb_domain::i18n::{Locale, fmt1};
use awb_domain::profile::AuthMethod;
use awb_domain::rules::RuleSet;
use awb_engine::general_fixes::FixRegistry;
use awb_engine::transform::TransformEngine;
use awb_mw_api::client::{MediaWikiClient, ReqwestMwClient};
use awb_security::{CredentialPort, InMemoryCredentialStore};
use awb_storage::TomlConfigStore;
use console::style;
use std::collections::HashSet;
use std::path::PathBuf;
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

pub async fn run(args: BotRunArgs) -> Result<()> {
    let cat = args.locale.catalog();
    println!("{}", style(cat.bot_title).bold().cyan());
    println!("{}: {}", cat.label_wiki, args.wiki);
    println!("{}: {}", cat.label_profile, args.profile_path.display());
    println!(
        "{}: {}",
        cat.label_mode,
        if args.dry_run {
            style(cat.mode_dry_run).yellow()
        } else {
            style(cat.mode_autonomous).green().bold()
        }
    );
    if let Some(max) = args.max_edits {
        println!("{}: {}", cat.label_max_edits, max);
    }
    println!();

    // Load profile
    let config_store = TomlConfigStore::new(&args.profile_path);
    let profile = config_store
        .load_profile(&args.auth_profile)
        .context(cat.err_load_profile)?;

    // Get credentials
    let cred_store = InMemoryCredentialStore::new();
    let password = cred_store
        .get_password(&args.auth_profile)
        .context(cat.err_no_stored_creds)?;

    // Create client and login
    let client = ReqwestMwClient::new(args.wiki.clone(), profile.throttle_policy.clone())
        .context(cat.err_create_http_client)?;

    print!("{}", cat.msg_logging_in);
    let username = match &profile.auth_method {
        AuthMethod::BotPassword { username } => username.clone(),
        AuthMethod::OAuth2 { .. } => {
            anyhow::bail!("{}", cat.err_oauth2_unimplemented);
        }
        AuthMethod::OAuth1 { .. } => {
            anyhow::bail!("{}", cat.err_oauth1_unimplemented);
        }
    };

    client
        .login_bot_password(&username, &password)
        .await
        .context(cat.err_login_failed)?;
    println!("{}", style("✓").green().bold());

    // Fetch CSRF token
    print!("{}", cat.msg_fetching_csrf);
    client
        .fetch_csrf_token()
        .await
        .context(cat.err_fetch_csrf)?;
    println!("{}", style("✓").green().bold());

    // Load rules and build engine
    let ruleset = RuleSet::new(); // In production, load from profile
    let registry = FixRegistry::with_defaults();
    let enabled_fixes = HashSet::new(); // In production, load from profile

    let engine =
        TransformEngine::new(&ruleset, registry, enabled_fixes).context(cat.err_create_engine)?;

    // For demo purposes, generate a simple page list
    // In real usage, this would come from the profile configuration or a list command
    let pages = vec![
        "Test Page 1".to_string(),
        "Test Page 2".to_string(),
        "Test Page 3".to_string(),
    ];

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
    let checkpoint = if let Some(ref path) = args.checkpoint_path {
        if path.exists() {
            println!(
                "{}",
                fmt1(
                    cat.msg_loading_checkpoint,
                    "path",
                    &path.display().to_string()
                )
            );
            Checkpoint::load(path).context(cat.err_load_checkpoint)?
        } else {
            Checkpoint::new()
        }
    } else {
        Checkpoint::new()
    };

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
    bot_runner.add_secret(password.clone());

    let report = match bot_runner.run().await {
        Ok(report) => report,
        Err(e) => {
            eprintln!(
                "{} {}",
                style("✗").red(),
                fmt1(cat.msg_bot_error, "error", &e.to_string())
            );

            // Save checkpoint on error
            if let Some(path) = args.checkpoint_path {
                if let Err(e) = bot_runner.save_checkpoint(&path) {
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
    if let Some(path) = args.checkpoint_path {
        bot_runner
            .save_checkpoint(&path)
            .context(cat.err_save_final_checkpoint)?;
    }

    // Display report
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", report.to_summary_localized(args.locale));
    println!("{}", style("═".repeat(60)).dim());

    // Save JSON report
    let report_path = PathBuf::from(format!(
        "bot-report-{}.json",
        chrono::Utc::now().format("%Y%m%d-%H%M%S")
    ));
    tokio::fs::write(&report_path, report.to_json()?)
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
