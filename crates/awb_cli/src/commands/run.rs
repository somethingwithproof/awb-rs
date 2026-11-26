// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Catalog, Locale, fmt1, fmt2};
use awb_domain::types::Title;
use awb_engine::diff_engine::{compute_diff, to_unified};
use awb_mw_api::client::{EditRequest, MediaWikiClient};
use chrono::Utc;
use console::style;
use dialoguer::Select;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use url::Url;

fn mode_label(batch: bool, dry_run: bool, cat: &Catalog) -> console::StyledObject<&'static str> {
    if dry_run {
        style(cat.mode_dry_run).yellow()
    } else if batch {
        style(cat.mode_batch).green()
    } else {
        style(cat.mode_interactive).cyan()
    }
}

pub async fn run(
    wiki: Url,
    profile_path: PathBuf,
    batch: bool,
    dry_run: bool,
    auth_profile: String,
    locale: Locale,
) -> Result<()> {
    let cat = locale.catalog();
    super::emit_stdout_line(&format!("{}", style(cat.run_title).bold().cyan()));
    super::emit_stdout_line(&format!("{}: {}", cat.label_wiki, wiki));
    super::emit_stdout_line(&format!(
        "{}: {}",
        cat.label_profile,
        profile_path.display()
    ));
    super::emit_stdout_line(&format!(
        "{}: {}",
        cat.label_mode,
        mode_label(batch, dry_run, cat)
    ));
    super::emit_stdout_line("");

    let (client, _password) =
        super::connect_authenticated_client(&wiki, &profile_path, &auth_profile, cat).await?;

    // For demo purposes, generate a simple page list
    // In real usage, this would come from the profile configuration
    let titles = vec![
        Title::new(awb_domain::types::Namespace::MAIN, "Test Page 1"),
        Title::new(awb_domain::types::Namespace::MAIN, "Test Page 2"),
    ];

    run_workflow(&client, batch, dry_run, &titles, cat).await
}

async fn run_workflow<C: MediaWikiClient>(
    client: &C,
    batch: bool,
    dry_run: bool,
    titles: &[Title],
    cat: &Catalog,
) -> Result<()> {
    println!();
    println!(
        "{}",
        fmt1(cat.msg_processing_pages, "count", &titles.len().to_string())
    );
    println!();

    let pb = ProgressBar::new(titles.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("valid progress template")
            .progress_chars("#>-"),
    );

    let mut saved_count = 0;
    let mut skipped_count = 0;

    for title in titles {
        pb.set_message(title.display.clone());

        // Fetch page
        let page = match client.get_page(title).await {
            Ok(p) => p,
            Err(e) => {
                pb.println(format!(
                    "  {} {}",
                    style("✗").red(),
                    fmt2(
                        cat.msg_failed_fetch_page,
                        "page",
                        &title.display,
                        "error",
                        &e.to_string()
                    )
                ));
                skipped_count += 1;
                pb.inc(1);
                continue;
            }
        };

        // Apply transformations (simplified - in real usage would use awb_engine rules)
        let new_text = apply_simple_transform(&page.wikitext);

        if new_text == page.wikitext {
            pb.println(format!(
                "  {} {}",
                style("→").dim(),
                fmt1(cat.msg_no_changes, "page", &title.display)
            ));
            skipped_count += 1;
            pb.inc(1);
            continue;
        }

        // Compute diff
        let diff_ops = compute_diff(&page.wikitext, &new_text);
        let unified_diff = to_unified(&diff_ops, 3);

        // Show diff
        pb.println(format!(
            "\n{}",
            style(fmt1(cat.msg_diff_for, "page", &title.display)).bold()
        ));
        pb.println(style("─".repeat(60)).dim().to_string());
        for line in unified_diff.lines().take(20) {
            if line.starts_with('+') {
                pb.println(style(line).green().to_string());
            } else if line.starts_with('-') {
                pb.println(style(line).red().to_string());
            } else {
                pb.println(line);
            }
        }
        pb.println(style("─".repeat(60)).dim().to_string());

        // Decide action
        let should_save = if dry_run {
            pb.println(format!(
                "  {} {}\n",
                style("ℹ").cyan(),
                cat.msg_dry_run_skip
            ));
            false
        } else if batch {
            pb.println(format!(
                "  {} {}\n",
                style("✓").green(),
                cat.msg_batch_autosave
            ));
            true
        } else {
            // Interactive mode
            let choices = vec![cat.action_save, cat.action_skip, cat.action_stop];
            let selection = Select::new()
                .with_prompt(cat.prompt_action)
                .items(&choices)
                .default(0)
                .interact()
                .context(cat.err_read_user_input)?;

            match selection {
                0 => true,  // Save
                1 => false, // Skip
                2 => {
                    // Stop
                    pb.println(format!("\n{}", style(cat.msg_stopped_by_user).yellow()));
                    break;
                }
                _ => false,
            }
        };

        if should_save {
            let edit_request = EditRequest {
                title: title.clone(),
                text: new_text,
                summary: "AWB-RS automated edit".to_string(),
                minor: true,
                bot: true,
                base_timestamp: page.timestamp.to_rfc3339(),
                start_timestamp: Utc::now().to_rfc3339(),
                section: None,
            };

            match client.edit_page(&edit_request).await {
                Ok(response) => {
                    pb.println(format!(
                        "  {} {}",
                        style("✓").green().bold(),
                        fmt2(
                            cat.msg_saved_page,
                            "page",
                            &title.display,
                            "rev",
                            &response.new_revid.unwrap_or(0).to_string()
                        )
                    ));
                    saved_count += 1;
                }
                Err(e) => {
                    pb.println(format!(
                        "  {} {}",
                        style("✗").red(),
                        fmt2(
                            cat.msg_failed_save_page,
                            "page",
                            &title.display,
                            "error",
                            &e.to_string()
                        )
                    ));
                    skipped_count += 1;
                }
            }
        } else {
            pb.println(format!(
                "  {} {}\n",
                style("→").yellow(),
                fmt1(cat.msg_skipped_page, "page", &title.display)
            ));
            skipped_count += 1;
        }

        pb.inc(1);
    }

    pb.finish_with_message(cat.msg_complete);

    println!();
    println!("{}", style(cat.label_summary).bold().cyan());
    println!(
        "  {}: {}",
        cat.label_saved,
        style(saved_count).green().bold()
    );
    println!("  {}: {}", cat.label_skipped, style(skipped_count).yellow());
    println!();

    Ok(())
}

fn apply_simple_transform(wikitext: &str) -> String {
    // Simple example transformation: fix double spaces
    wikitext.replace("  ", " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_support::{MockMwClient, TestTempDir, write_test_profile};
    use awb_domain::types::Namespace;

    #[test]
    fn test_mode_label_variants() {
        for locale in [Locale::En, Locale::De] {
            let cat = locale.catalog();
            assert!(
                mode_label(false, true, cat)
                    .to_string()
                    .contains(cat.mode_dry_run)
            );
            assert!(
                mode_label(true, false, cat)
                    .to_string()
                    .contains(cat.mode_batch)
            );
            assert!(
                mode_label(false, false, cat)
                    .to_string()
                    .contains(cat.mode_interactive)
            );
        }
    }

    #[test]
    fn test_apply_simple_transform() {
        assert_eq!(apply_simple_transform("Hello  world"), "Hello world");
        assert_eq!(apply_simple_transform("No changes"), "No changes");
    }

    #[tokio::test]
    async fn test_run_missing_profile_and_missing_creds() {
        let tmp = TestTempDir::new("run");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();

        let missing = tmp.path().join("nonexistent.toml");
        for (batch, dry_run, locale) in [
            (false, true, Locale::En),
            (true, false, Locale::De),
            (false, false, Locale::Ja),
        ] {
            let cat = locale.catalog();
            let err = run(
                wiki.clone(),
                missing.clone(),
                batch,
                dry_run,
                "default".to_string(),
                locale,
            )
            .await
            .unwrap_err();
            assert!(err.to_string().contains(cat.err_load_profile));
        }

        let valid = tmp.path().join("profiles.toml");
        write_test_profile(&valid, "bot-main", &wiki);
        let cat = Locale::Es.catalog();
        let err = run(wiki, valid, false, true, "bot-main".to_string(), Locale::Es)
            .await
            .unwrap_err();
        assert!(err.to_string().contains(cat.err_no_stored_creds));
    }

    #[tokio::test]
    async fn test_run_workflow_dry_run_and_batch_with_mock_client() {
        let mut client = MockMwClient::new();
        client.add_page("Changed", "Line 1\nHello  world\nLine 3");
        client.add_page("Unchanged", "Already clean");

        let titles = vec![
            Title::new(Namespace::MAIN, "Changed"),
            Title::new(Namespace::MAIN, "Unchanged"),
            Title::new(Namespace::MAIN, "Missing"),
        ];

        // Dry-run exercises diff rendering, no-change, fetch error, and dry-run skip
        run_workflow(&client, false, true, &titles, Locale::De.catalog())
            .await
            .expect("dry-run workflow succeeds");

        // Batch mode exercises auto-save success
        run_workflow(&client, true, false, &titles, Locale::Ja.catalog())
            .await
            .expect("batch workflow succeeds");

        // Batch mode with edit failure exercises failed save path
        client.fail_edit = true;
        run_workflow(&client, true, false, &titles, Locale::Fr.catalog())
            .await
            .expect("batch workflow handles edit error");
    }
}
