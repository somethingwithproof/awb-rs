// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Catalog, Locale, fmt1};
use awb_mw_api::client::{MediaWikiClient, ReqwestMwClient};
use awb_security::{CredentialPort, KeyringCredentialStore};
use console::style;
use dialoguer::Password;
use url::Url;

fn print_login_header(wiki: &Url, username: &str, profile: &str, cat: &Catalog) {
    super::emit_stdout_line(&format!("{}", style(cat.login_title).bold().cyan()));
    super::emit_stdout_line(&format!("{}: {}", cat.label_wiki, wiki));
    super::emit_stdout_line(&format!("{}: {}", cat.label_account, username));
    super::emit_stdout_line(&format!("{}: {}", cat.label_profile, profile));
    super::emit_stdout_line("");
}

pub async fn run(wiki: Url, username: String, profile: String, locale: Locale) -> Result<()> {
    let cat = locale.catalog();
    print_login_header(&wiki, &username, &profile, cat);

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

    super::emit_stdout_line(&format!("{}", style("✓").green().bold()));

    // Store credentials in OS keychain
    let cred_store = KeyringCredentialStore::new();
    cred_store
        .set_password(&profile, &password)
        .context(cat.err_store_keychain)?;

    super::emit_stdout_line("");
    super::emit_stdout_line(&format!("{}", style(cat.msg_login_success).green().bold()));
    super::emit_stdout_line(&fmt1(
        cat.msg_creds_stored_profile,
        "profile",
        &style(&profile).yellow().to_string(),
    ));
    super::emit_stdout_line("");
    super::emit_stdout_line(cat.msg_creds_saved_keychain);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_login_header() {
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();
        for locale in [Locale::En, Locale::De, Locale::ArEg] {
            print_login_header(&wiki, "ExampleBot", "default", locale.catalog());
        }
    }
}
