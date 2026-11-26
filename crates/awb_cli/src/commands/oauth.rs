// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Catalog, Locale, fmt1};
use awb_domain::profile::Profile;
use awb_security::{CredentialPort, KeyringCredentialStore};
use dialoguer::{Input, Password};
use secrecy::SecretString;
use std::path::{Path, PathBuf};
use url::Url;

pub(crate) fn validate_profile_name(profile: &str, cat: &Catalog) -> Result<()> {
    if profile.is_empty()
        || !profile
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("{}", cat.err_invalid_profile_name);
    }
    Ok(())
}

async fn save_profile_to_dir(
    dir: &Path,
    profile: &str,
    profile_obj: &Profile,
    cat: &Catalog,
) -> Result<PathBuf> {
    let profile_path = dir.join(format!("{profile}.toml"));
    tokio::fs::create_dir_all(dir)
        .await
        .context(cat.err_create_profiles_dir)?;
    let profile_toml = toml::to_string_pretty(profile_obj).context(cat.err_serialize_profile)?;
    tokio::fs::write(&profile_path, profile_toml)
        .await
        .with_context(|| {
            fmt1(
                cat.err_write_profile,
                "path",
                &profile_path.display().to_string(),
            )
        })?;
    Ok(profile_path)
}

pub async fn setup(
    wiki: Url,
    consumer_key: String,
    access_token: String,
    profile: String,
    locale: Locale,
) -> Result<()> {
    use awb_domain::profile::{AuthMethod, ThrottlePolicy};
    let cat = locale.catalog();

    // Validate profile name to prevent path traversal
    validate_profile_name(&profile, cat)?;

    super::emit_stdout_line(&fmt1(cat.msg_oauth1_setup, "wiki", wiki.as_str()));

    // Prompt for secrets interactively (never via CLI args)
    let consumer_secret = Password::new()
        .with_prompt(cat.prompt_consumer_secret)
        .interact()
        .context(cat.err_read_consumer_secret)?;

    let access_secret = Password::new()
        .with_prompt(cat.prompt_access_secret)
        .interact()
        .context(cat.err_read_access_secret)?;

    // Create profile with OAuth1 auth
    let auth_method = AuthMethod::OAuth1 {
        consumer_key: consumer_key.clone(),
        consumer_secret: SecretString::new(consumer_secret.clone().into()),
        access_token: access_token.clone(),
        access_secret: SecretString::new(access_secret.clone().into()),
    };

    let profile_obj = Profile {
        id: profile.clone(),
        name: format!("OAuth Profile for {}", wiki.host_str().unwrap_or("unknown")),
        api_url: wiki.clone(),
        auth_method,
        default_namespaces: std::collections::HashSet::new(),
        throttle_policy: ThrottlePolicy::default(),
    };

    // Store OAuth credentials in OS keychain
    let store = KeyringCredentialStore::new();
    let token_json = serde_json::json!({
        "consumer_key": consumer_key,
        "consumer_secret": consumer_secret,
        "access_token": access_token,
        "access_secret": access_secret,
    })
    .to_string();

    store
        .set_oauth_token(&profile, &token_json)
        .context(cat.err_store_oauth_keychain)?;

    // Save profile
    let profile_path =
        save_profile_to_dir(Path::new(".awb/profiles"), &profile, &profile_obj, cat).await?;

    super::emit_stdout_line(&format!(
        "✓ {}",
        fmt1(cat.msg_oauth1_saved, "profile", &profile)
    ));
    super::emit_stdout_line(&format!(
        "✓ {}",
        fmt1(
            cat.msg_profile_saved,
            "path",
            &profile_path.display().to_string()
        )
    ));

    Ok(())
}

pub async fn authorize(
    wiki: Url,
    client_id: String,
    profile: String,
    locale: Locale,
) -> Result<()> {
    use awb_mw_api::oauth::{OAuth2Config, oauth2_authorization_url, oauth2_exchange_code};
    let cat = locale.catalog();

    // Validate profile name to prevent path traversal
    validate_profile_name(&profile, cat)?;

    super::emit_stdout_line(&fmt1(cat.msg_oauth2_start, "wiki", wiki.as_str()));

    // Prompt for client secret interactively (never via CLI args)
    let client_secret = Password::new()
        .with_prompt(cat.prompt_client_secret)
        .interact()
        .context(cat.err_read_client_secret)?;

    // Build OAuth2 config
    // Note: These endpoints are MediaWiki-specific and may need to be customized
    let auth_endpoint = format!(
        "{}?title=Special:OAuth/authorize",
        wiki.origin().ascii_serialization()
    );
    let token_endpoint = format!(
        "{}?title=Special:OAuth/token",
        wiki.origin().ascii_serialization()
    );
    let redirect_uri = "http://localhost:8080/callback".to_string();

    let config = OAuth2Config {
        client_id: client_id.clone(),
        client_secret: client_secret.clone().into(),
        redirect_uri: redirect_uri.clone(),
        token_endpoint,
        auth_endpoint,
    };

    // Generate authorization URL with PKCE
    let (auth_url, state, pkce_verifier) = oauth2_authorization_url(&config)
        .await
        .context(cat.err_generate_auth_url)?;

    super::emit_stdout_line(&format!("\n🔐 {}", cat.msg_visit_auth_url));
    super::emit_stdout_line(&format!("{}\n", auth_url));
    super::emit_stdout_line(cat.msg_callback_redirect_info);
    super::emit_stdout_line(&format!("{}\n", cat.msg_callback_paste_code));

    let code: String = Input::new()
        .with_prompt(cat.prompt_auth_code)
        .interact_text()
        .context(cat.err_read_auth_code)?;

    let received_state: String = Input::new()
        .with_prompt(cat.prompt_state_param)
        .interact_text()
        .context(cat.err_read_state)?;

    // Exchange code for token with PKCE verifier
    super::emit_stdout_line(cat.msg_exchanging_code);
    let token = oauth2_exchange_code(&config, &code, &state, &received_state, &pkce_verifier)
        .await
        .context(cat.err_exchange_code)?;

    // Store tokens in OS keychain
    let store = KeyringCredentialStore::new();
    let token_json = serde_json::to_string(&token).context(cat.err_serialize_token)?;
    store
        .set_oauth_token(&profile, &token_json)
        .context(cat.err_store_token_keychain)?;

    // Create and save profile
    use awb_domain::profile::{AuthMethod, ThrottlePolicy};
    let auth_method = AuthMethod::OAuth2 {
        client_id,
        client_secret: SecretString::new(client_secret.into()),
    };

    let profile_obj = Profile {
        id: profile.clone(),
        name: format!(
            "OAuth2 Profile for {}",
            wiki.host_str().unwrap_or("unknown")
        ),
        api_url: wiki.clone(),
        auth_method,
        default_namespaces: std::collections::HashSet::new(),
        throttle_policy: ThrottlePolicy::default(),
    };

    let profile_path =
        save_profile_to_dir(Path::new(".awb/profiles"), &profile, &profile_obj, cat).await?;

    super::emit_stdout_line(&format!(
        "✓ {}",
        fmt1(cat.msg_oauth2_saved, "profile", &profile)
    ));
    super::emit_stdout_line(&format!(
        "✓ {}",
        fmt1(
            cat.msg_profile_saved,
            "path",
            &profile_path.display().to_string()
        )
    ));

    if let Some(expires) = token.expires_in {
        super::emit_stdout_line(&format!(
            "ℹ {}",
            fmt1(cat.msg_token_expires, "seconds", &expires.to_string())
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_support::TestTempDir;
    use awb_domain::profile::{AuthMethod, ThrottlePolicy};
    use std::collections::HashSet;

    #[test]
    fn test_validate_profile_name() {
        let cat = Locale::De.catalog();
        assert!(validate_profile_name("default", cat).is_ok());
        assert!(validate_profile_name("my_bot-01", cat).is_ok());

        for bad in ["", "../escape", "dir/file", "bad name", "semi;colon"] {
            let err = validate_profile_name(bad, cat).unwrap_err();
            assert!(
                err.to_string().contains(cat.err_invalid_profile_name),
                "expected invalid profile error for {bad:?}, got {err}"
            );
        }
    }

    #[tokio::test]
    async fn test_setup_and_authorize_reject_invalid_profile_names() {
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();

        let err_setup = setup(
            wiki.clone(),
            "ck".to_string(),
            "at".to_string(),
            "../evil".to_string(),
            Locale::Fr,
        )
        .await
        .unwrap_err();
        assert!(
            err_setup
                .to_string()
                .contains(Locale::Fr.catalog().err_invalid_profile_name)
        );

        let err_auth = authorize(
            wiki,
            "cid".to_string(),
            "bad/profile".to_string(),
            Locale::Ja,
        )
        .await
        .unwrap_err();
        assert!(
            err_auth
                .to_string()
                .contains(Locale::Ja.catalog().err_invalid_profile_name)
        );
    }

    #[tokio::test]
    async fn test_save_profile_to_dir_success_and_errors() {
        let tmp = TestTempDir::new("oauth-save");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();
        let cat = Locale::En.catalog();
        let profile = Profile {
            id: "test_oauth".to_string(),
            name: "OAuth Test".to_string(),
            api_url: wiki,
            auth_method: AuthMethod::OAuth2 {
                client_id: "cid".to_string(),
                client_secret: SecretString::new("secret".to_string().into()),
            },
            default_namespaces: HashSet::new(),
            throttle_policy: ThrottlePolicy::default(),
        };

        let profiles_dir = tmp.path().join("profiles");
        let saved = save_profile_to_dir(&profiles_dir, "test_oauth", &profile, cat)
            .await
            .expect("save profile succeeds");
        assert!(saved.is_file());
        let content = std::fs::read_to_string(&saved).expect("read saved profile");
        assert!(content.contains("test_oauth"));

        // Target path is an existing directory -> tokio::fs::write fails with localized fmt1 context
        let blocking_dir = profiles_dir.join("dir_conflict.toml");
        std::fs::create_dir_all(&blocking_dir).expect("create blocking dir");
        let err = save_profile_to_dir(&profiles_dir, "dir_conflict", &profile, cat)
            .await
            .unwrap_err();
        assert!(
            err.to_string()
                .contains(&blocking_dir.display().to_string()),
            "expected path in error context, got: {err}"
        );

        // Parent dir is a regular file -> create_dir_all fails with err_create_profiles_dir
        let file_as_dir = tmp.path().join("not_a_dir");
        std::fs::write(&file_as_dir, b"x").expect("write marker file");
        let err = save_profile_to_dir(&file_as_dir, "any", &profile, cat)
            .await
            .unwrap_err();
        assert!(err.to_string().contains(cat.err_create_profiles_dir));
    }
}
