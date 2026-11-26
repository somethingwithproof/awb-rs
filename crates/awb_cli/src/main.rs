// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use awb_domain::i18n::{Locale, extract_cli_lang_os};
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use std::path::PathBuf;
use url::Url;

mod commands;

#[derive(Parser)]
#[command(name = "awb-rs")]
#[command(version, about = "AutoWikiBrowser in Rust - Wikipedia bot framework", long_about = None)]
struct Cli {
    /// Display language (e.g. en, de, ja, zh-CN, pt-BR)
    #[arg(long, global = true)]
    lang: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to a MediaWiki instance
    Login {
        /// Wiki API URL (e.g., https://en.wikipedia.org/w/api.php)
        #[arg(long)]
        wiki: Url,

        /// Bot username
        #[arg(long)]
        username: String,

        /// Profile ID to save credentials under
        #[arg(long, default_value = "default")]
        profile: String,
    },

    /// List pages from various sources
    List {
        /// Wiki API URL
        #[arg(long)]
        wiki: Url,

        /// Source type
        #[arg(long)]
        source: ListSource,

        /// Query value (category name, page title, search query, or file path)
        #[arg(long)]
        query: String,

        /// Maximum number of pages to fetch (0 = unlimited)
        #[arg(long, default_value = "100")]
        limit: usize,
    },

    /// Run editing workflow with a profile
    Run {
        /// Wiki API URL
        #[arg(long)]
        wiki: Url,

        /// Profile file path (TOML)
        #[arg(long)]
        profile: PathBuf,

        /// Batch mode (auto-save all changes)
        #[arg(long)]
        batch: bool,

        /// Dry-run mode (show diffs without saving)
        #[arg(long)]
        dry_run: bool,

        /// Profile ID for credentials
        #[arg(long, default_value = "default")]
        auth_profile: String,
    },

    /// Export telemetry log
    ExportLog {
        /// Output format
        #[arg(long)]
        format: ExportFormat,

        /// Output file path
        #[arg(long)]
        output: PathBuf,
    },

    /// Run bot mode (unattended batch editing)
    Bot {
        /// Wiki API URL
        #[arg(long)]
        wiki: Url,

        /// Profile file path (TOML)
        #[arg(long)]
        profile: PathBuf,

        /// Maximum number of edits (default: unlimited)
        #[arg(long)]
        max_edits: Option<u32>,

        /// Dry-run mode (show diffs without saving)
        #[arg(long)]
        dry_run: bool,

        /// Checkpoint file path for resume capability
        #[arg(long)]
        checkpoint: Option<PathBuf>,

        /// Profile ID for credentials
        #[arg(long, default_value = "default")]
        auth_profile: String,

        /// Skip pages with no changes
        #[arg(long, default_value = "true")]
        skip_no_change: bool,

        /// Skip pages with warnings
        #[arg(long)]
        skip_on_warning: bool,

        /// Log progress every N pages
        #[arg(long, default_value = "10")]
        log_every_n: u32,
    },

    /// OAuth authentication management
    #[command(subcommand)]
    OAuth(OAuthCommands),
}

#[derive(Subcommand)]
enum OAuthCommands {
    /// Setup OAuth 1.0a credentials
    Setup {
        /// Wiki API URL
        #[arg(long)]
        wiki: Url,

        /// OAuth consumer key
        #[arg(long)]
        consumer_key: String,

        /// OAuth access token
        #[arg(long)]
        access_token: String,

        /// Profile ID to save credentials under
        #[arg(long, default_value = "default")]
        profile: String,
    },

    /// Authorize OAuth 2.0 (opens browser)
    Authorize {
        /// Wiki API URL
        #[arg(long)]
        wiki: Url,

        /// OAuth 2.0 client ID
        #[arg(long)]
        client_id: String,

        /// Profile ID to save credentials under
        #[arg(long, default_value = "default")]
        profile: String,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum ListSource {
    Category,
    WhatLinksHere,
    Search,
    File,
    Watchlist,
    UserContribs,
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum ExportFormat {
    Json,
    Csv,
    Plain,
}

fn localized_command(locale: Locale) -> clap::Command {
    let cat = locale.catalog();
    Cli::command()
        .about(cat.cli_about)
        .mut_arg("lang", |a| a.help(cat.cli_opt_lang))
        .mut_subcommand("login", |c| c.about(cat.cli_cmd_login))
        .mut_subcommand("list", |c| c.about(cat.cli_cmd_list))
        .mut_subcommand("run", |c| c.about(cat.cli_cmd_run))
        .mut_subcommand("export-log", |c| c.about(cat.cli_cmd_export_log))
        .mut_subcommand("bot", |c| c.about(cat.cli_cmd_bot))
        .mut_subcommand("o-auth", |c| {
            c.about(cat.cli_cmd_oauth)
                .mut_subcommand("setup", |s| s.about(cat.cli_cmd_oauth_setup))
                .mut_subcommand("authorize", |s| s.about(cat.cli_cmd_oauth_authorize))
        })
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize telemetry
    awb_telemetry::init_telemetry(&awb_telemetry::TelemetryConfig {
        log_dir: "logs".into(),
        level: tracing::Level::INFO,
        json_output: true,
        human_output: true,
    })?;

    let raw_args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    let initial_locale = Locale::resolve(extract_cli_lang_os(&raw_args).as_deref());
    let matches = localized_command(initial_locale).get_matches_from(raw_args);
    let cli = Cli::from_arg_matches(&matches)?;
    let locale = Locale::resolve(cli.lang.as_deref());

    match cli.command {
        Commands::Login {
            wiki,
            username,
            profile,
        } => commands::login::run(wiki, username, profile, locale).await,
        Commands::List {
            wiki,
            source,
            query,
            limit,
        } => commands::list::run(wiki, source, query, limit, locale).await,
        Commands::Run {
            wiki,
            profile,
            batch,
            dry_run,
            auth_profile,
        } => commands::run::run(wiki, profile, batch, dry_run, auth_profile, locale).await,
        Commands::ExportLog { format, output } => {
            commands::export::run(format, output, locale).await
        }
        Commands::Bot {
            wiki,
            profile,
            max_edits,
            dry_run,
            checkpoint,
            auth_profile,
            skip_no_change,
            skip_on_warning,
            log_every_n,
        } => {
            commands::bot::run(commands::bot::BotRunArgs {
                wiki,
                profile_path: profile,
                max_edits,
                dry_run,
                checkpoint_path: checkpoint,
                auth_profile,
                skip_no_change,
                skip_on_warning,
                log_every_n,
                locale,
            })
            .await
        }
        Commands::OAuth(oauth_cmd) => match oauth_cmd {
            OAuthCommands::Setup {
                wiki,
                consumer_key,
                access_token,
                profile,
            } => commands::oauth::setup(wiki, consumer_key, access_token, profile, locale).await,
            OAuthCommands::Authorize {
                wiki,
                client_id,
                profile,
            } => commands::oauth::authorize(wiki, client_id, profile, locale).await,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localized_command_subcommands_across_locales() {
        for locale in [Locale::En, Locale::De, Locale::Ja, Locale::ArEg] {
            let cat = locale.catalog();
            let cmd = localized_command(locale);

            assert_eq!(
                cmd.get_about().map(|s| s.to_string()),
                Some(cat.cli_about.to_string())
            );

            let lang_arg = cmd
                .get_arguments()
                .find(|a| a.get_id() == "lang")
                .expect("lang argument present");
            assert_eq!(
                lang_arg.get_help().map(|s| s.to_string()),
                Some(cat.cli_opt_lang.to_string())
            );

            for (name, expected_about) in [
                ("login", cat.cli_cmd_login),
                ("list", cat.cli_cmd_list),
                ("run", cat.cli_cmd_run),
                ("export-log", cat.cli_cmd_export_log),
                ("bot", cat.cli_cmd_bot),
            ] {
                let sub = cmd.find_subcommand(name).expect("subcommand present");
                assert_eq!(
                    sub.get_about().map(|s| s.to_string()),
                    Some(expected_about.to_string())
                );
            }

            let oauth = cmd
                .find_subcommand("o-auth")
                .expect("o-auth subcommand present");
            assert_eq!(
                oauth.get_about().map(|s| s.to_string()),
                Some(cat.cli_cmd_oauth.to_string())
            );
            assert_eq!(
                oauth
                    .find_subcommand("setup")
                    .and_then(|s| s.get_about())
                    .map(|s| s.to_string()),
                Some(cat.cli_cmd_oauth_setup.to_string())
            );
            assert_eq!(
                oauth
                    .find_subcommand("authorize")
                    .and_then(|s| s.get_about())
                    .map(|s| s.to_string()),
                Some(cat.cli_cmd_oauth_authorize.to_string())
            );
        }
    }

    #[test]
    fn test_localized_command_parses_cli_args() {
        let matches = localized_command(Locale::De)
            .try_get_matches_from([
                "awb-rs",
                "--lang",
                "de",
                "export-log",
                "--format",
                "json",
                "--output",
                "out.json",
            ])
            .expect("valid cli arguments");
        let cli = Cli::from_arg_matches(&matches).expect("valid Cli match");
        assert_eq!(cli.lang.as_deref(), Some("de"));
        match cli.command {
            Commands::ExportLog { format, output } => {
                assert!(matches!(format, ExportFormat::Json));
                assert_eq!(output, PathBuf::from("out.json"));
            }
            _ => panic!("expected ExportLog command"),
        }
    }
}
