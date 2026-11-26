// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use awb_domain::i18n::{Locale, fmt1};
use awb_security::{CredentialPort, KeyringCredentialStore};
use dialoguer::{Input, Password};
use secrecy::SecretString;
use url::Url;

pub async fn setup(
    wiki: Url,
    consumer_key: String,
    access_token: String,
    profile: String,
    locale: Locale,
) -> Result<()> {
    use awb_domain::profile::{AuthMethod, Profile, ThrottlePolicy};
    let cat = locale.catalog();

    // Validate profile name to prevent path traversal
    if !profile
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("{}", cat.err_invalid_profile_name);
    }

    println!("{}", fmt1(cat.msg_oauth1_setup, "wiki", wiki.as_str()));

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
    let profile_path = format!(".awb/profiles/{}.toml", profile);
    tokio::fs::create_dir_all(".awb/profiles")
        .await
        .context(cat.err_create_profiles_dir)?;
    let profile_toml = toml::to_string_pretty(&profile_obj).context(cat.err_serialize_profile)?;
    tokio::fs::write(&profile_path, profile_toml)
        .await
        .context(fmt1(cat.err_write_profile, "path", &profile_path))?;

    println!("✓ {}", fmt1(cat.msg_oauth1_saved, "profile", &profile));
    println!("✓ {}", fmt1(cat.msg_profile_saved, "path", &profile_path));

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
    if !profile
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("{}", cat.err_invalid_profile_name);
    }

    println!("{}", fmt1(cat.msg_oauth2_start, "wiki", wiki.as_str()));

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

    println!("\n🔐 {}", cat.msg_visit_auth_url);
    println!("{}\n", auth_url);
    println!("{}", cat.msg_oauth2_redirect_info);
    println!("{}\n", cat.msg_oauth2_paste_code);

    let code: String = Input::new()
        .with_prompt(cat.prompt_auth_code)
        .interact_text()
        .context(cat.err_read_auth_code)?;

    let received_state: String = Input::new()
        .with_prompt(cat.prompt_state_param)
        .interact_text()
        .context(cat.err_read_state)?;

    // Exchange code for token with PKCE verifier
    println!("{}", cat.msg_exchanging_code);
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
    use awb_domain::profile::{AuthMethod, Profile, ThrottlePolicy};
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

    let profile_path = format!(".awb/profiles/{}.toml", profile);
    tokio::fs::create_dir_all(".awb/profiles")
        .await
        .context(cat.err_create_profiles_dir)?;
    let profile_toml = toml::to_string_pretty(&profile_obj).context(cat.err_serialize_profile)?;
    tokio::fs::write(&profile_path, profile_toml)
        .await
        .context(fmt1(cat.err_write_profile, "path", &profile_path))?;

    println!("✓ {}", fmt1(cat.msg_oauth2_saved, "profile", &profile));
    println!("✓ {}", fmt1(cat.msg_profile_saved, "path", &profile_path));

    if let Some(expires) = token.expires_in {
        println!(
            "ℹ {}",
            fmt1(cat.msg_token_expires, "seconds", &expires.to_string())
        );
    }

    Ok(())
}
