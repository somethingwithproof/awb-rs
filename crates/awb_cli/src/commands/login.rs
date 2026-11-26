// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Locale, fmt1};
use awb_mw_api::client::{MediaWikiClient, ReqwestMwClient};
use awb_security::{CredentialPort, KeyringCredentialStore};
use console::style;
use dialoguer::Password;
use url::Url;

pub async fn run(wiki: Url, username: String, profile: String, locale: Locale) -> Result<()> {
    let cat = locale.catalog();
    println!("{}", style(cat.login_title).bold().cyan());
    println!("{}: {}", cat.label_wiki, wiki);
    println!("{}: {}", cat.label_username, username);
    println!("{}: {}", cat.label_profile, profile);
    println!();

    // Prompt for password
    let password = Password::new()
        .with_prompt(cat.prompt_bot_password)
        .interact()
        .context(cat.err_read_password)?;

    // Create client and attempt login
    let client = ReqwestMwClient::new(wiki.clone(), awb_domain::profile::ThrottlePolicy::default())
        .context(cat.err_create_http_client)?;

    print!("{}", cat.msg_authenticating);
    client
        .login_bot_password(&username, &password)
        .await
        .context(cat.err_auth_failed)?;

    println!("{}", style("✓").green().bold());

    // Store credentials in OS keychain
    let cred_store = KeyringCredentialStore::new();
    cred_store
        .set_password(&profile, &password)
        .context(cat.err_store_keychain)?;

    println!();
    println!("{}", style(cat.msg_login_success).green().bold());
    println!(
        "{}",
        fmt1(
            cat.msg_creds_stored_profile,
            "profile",
            &style(&profile).yellow().to_string()
        )
    );
    println!();
    println!("{}", cat.msg_creds_saved_keychain);

    Ok(())
}
