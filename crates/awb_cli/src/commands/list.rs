// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Catalog, Locale, fmt1, fmt2};
use awb_domain::types::Title;
use awb_mw_api::list_endpoints::{fetch_all_pages, fetch_user_contributions, fetch_watchlist};
use console::style;
use url::Url;

use crate::ListSource;

pub async fn run(
    wiki: Url,
    source: ListSource,
    query: String,
    limit: usize,
    locale: Locale,
) -> Result<()> {
    let cat = locale.catalog();
    super::emit_stdout_line(&format!("{}", style(cat.list_title).bold().cyan()));
    super::emit_stdout_line(&format!("{}: {}", cat.label_wiki, wiki));
    super::emit_stdout_line(&format!("{}: {:?}", cat.label_source, source));
    if !matches!(source, ListSource::Watchlist) {
        super::emit_stdout_line(&format!("{}: {}", cat.label_query, query));
    }
    super::emit_stdout_line("");

    let titles = match source {
        ListSource::Category => fetch_category_members(&wiki, &query, limit, cat).await?,
        ListSource::WhatLinksHere => fetch_what_links_here(&wiki, &query, limit, cat).await?,
        ListSource::Search => fetch_search_results(&wiki, &query, limit, cat).await?,
        ListSource::File => fetch_from_file(&query, cat).await?,
        ListSource::Watchlist => fetch_watchlist_pages(&wiki, limit, cat).await?,
        ListSource::UserContribs => fetch_user_contribs(&wiki, &query, limit, cat).await?,
    };

    println!(
        "{} {}",
        style("✓").green().bold(),
        fmt1(
            cat.msg_found_pages,
            "count",
            &style(titles.len()).yellow().bold().to_string()
        )
    );
    println!();

    for (i, title) in titles.iter().enumerate().take(limit.max(1)) {
        println!("  {}. {}", style(i + 1).dim(), title.display);
    }

    if titles.len() > limit && limit > 0 {
        println!();
        println!(
            "  {} {}",
            style("...").dim(),
            fmt2(
                cat.msg_showing_first,
                "shown",
                &limit.to_string(),
                "total",
                &titles.len().to_string()
            )
        );
    }

    Ok(())
}

async fn fetch_category_members(
    api_url: &Url,
    category: &str,
    limit: usize,
    cat: &Catalog,
) -> Result<Vec<Title>> {
    let client = reqwest::Client::builder()
        .user_agent("AWB-RS/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let category_title = if category.starts_with("Category:") {
        category.to_string()
    } else {
        format!("Category:{}", category)
    };

    let base_params = [
        ("action", "query"),
        ("list", "categorymembers"),
        ("cmtitle", &category_title),
        ("cmlimit", "500"),
    ];

    let mut titles = fetch_all_pages(
        &client,
        api_url,
        &base_params,
        "categorymembers",
        "cmcontinue",
    )
    .await
    .context(cat.err_fetch_category)?;

    if limit > 0 && titles.len() > limit {
        titles.truncate(limit);
    }

    Ok(titles)
}

async fn fetch_what_links_here(
    api_url: &Url,
    page: &str,
    limit: usize,
    cat: &Catalog,
) -> Result<Vec<Title>> {
    let client = reqwest::Client::builder()
        .user_agent("AWB-RS/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let base_params = [
        ("action", "query"),
        ("list", "backlinks"),
        ("bltitle", page),
        ("bllimit", "500"),
    ];

    let mut titles = fetch_all_pages(&client, api_url, &base_params, "backlinks", "blcontinue")
        .await
        .context(cat.err_fetch_backlinks)?;

    if limit > 0 && titles.len() > limit {
        titles.truncate(limit);
    }

    Ok(titles)
}

async fn fetch_search_results(
    api_url: &Url,
    search_query: &str,
    limit: usize,
    cat: &Catalog,
) -> Result<Vec<Title>> {
    let client = reqwest::Client::builder()
        .user_agent("AWB-RS/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let base_params = [
        ("action", "query"),
        ("list", "search"),
        ("srsearch", search_query),
        ("srlimit", "500"),
    ];

    let mut titles = fetch_all_pages(&client, api_url, &base_params, "search", "sroffset")
        .await
        .context(cat.err_fetch_search)?;

    if limit > 0 && titles.len() > limit {
        titles.truncate(limit);
    }

    Ok(titles)
}

async fn fetch_from_file(file_path: &str, cat: &Catalog) -> Result<Vec<Title>> {
    // Verify file exists and is a regular file (not a symlink)
    let metadata = tokio::fs::metadata(file_path)
        .await
        .context(cat.err_access_file)?;

    if !metadata.is_file() {
        anyhow::bail!("{}", cat.err_not_regular_file);
    }

    // Check file size (reject files >10MB)
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
    if metadata.len() > MAX_FILE_SIZE {
        anyhow::bail!("{}", cat.err_file_too_large);
    }

    let content = tokio::fs::read_to_string(file_path)
        .await
        .context(cat.err_read_file)?;

    let titles: Vec<Title> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim();
            awb_domain::types::Title::new(awb_domain::types::Namespace::MAIN, trimmed)
        })
        .collect();

    Ok(titles)
}

async fn fetch_watchlist_pages(api_url: &Url, limit: usize, cat: &Catalog) -> Result<Vec<Title>> {
    let client = reqwest::Client::builder()
        .user_agent("AWB-RS/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let titles = fetch_watchlist(&client, api_url, limit as u32)
        .await
        .context(cat.err_fetch_watchlist)?;

    Ok(titles)
}

async fn fetch_user_contribs(
    api_url: &Url,
    username: &str,
    limit: usize,
    cat: &Catalog,
) -> Result<Vec<Title>> {
    let client = reqwest::Client::builder()
        .user_agent("AWB-RS/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let titles = fetch_user_contributions(&client, api_url, username, limit as u32)
        .await
        .context(cat.err_fetch_contribs)?;

    Ok(titles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_support::TestTempDir;

    #[tokio::test]
    async fn test_list_run_from_file_valid_and_limited() {
        let tmp = TestTempDir::new("list");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();

        let list_file = tmp.path().join("pages.txt");
        std::fs::write(&list_file, "Alpha\n\n  Beta  \nGamma\nDelta\n").expect("write pages file");

        run(
            wiki.clone(),
            ListSource::File,
            list_file.display().to_string(),
            10,
            Locale::En,
        )
        .await
        .expect("list from file succeeds");

        // Trigger the `titles.len() > limit && limit > 0` truncation message branch
        run(
            wiki,
            ListSource::File,
            list_file.display().to_string(),
            2,
            Locale::De,
        )
        .await
        .expect("list from file with limit succeeds");

        let titles = fetch_from_file(&list_file.display().to_string(), Locale::En.catalog())
            .await
            .expect("fetch_from_file returns parsed titles");
        assert_eq!(titles.len(), 4);
        assert_eq!(titles[0].display, "Alpha");
        assert_eq!(titles[1].display, "Beta");
    }

    #[tokio::test]
    async fn test_list_run_from_file_errors() {
        let tmp = TestTempDir::new("list-err");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();

        // Nonexistent file
        let missing = tmp.path().join("does-not-exist.txt");
        let err = run(
            wiki.clone(),
            ListSource::File,
            missing.display().to_string(),
            10,
            Locale::Fr,
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string()
                .contains(Locale::Fr.catalog().err_access_file)
        );

        // Directory instead of regular file
        let err = run(
            wiki.clone(),
            ListSource::File,
            tmp.path().display().to_string(),
            10,
            Locale::Es,
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string()
                .contains(Locale::Es.catalog().err_not_regular_file)
        );

        // Oversized (>10MB) sparse file
        let huge = tmp.path().join("huge.txt");
        let file = std::fs::File::create(&huge).expect("create huge file");
        file.set_len(10 * 1024 * 1024 + 1)
            .expect("set sparse file length");
        drop(file);
        let err = run(
            wiki,
            ListSource::File,
            huge.display().to_string(),
            10,
            Locale::Ja,
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string()
                .contains(Locale::Ja.catalog().err_file_too_large)
        );
    }
}
