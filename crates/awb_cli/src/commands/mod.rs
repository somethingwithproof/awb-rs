// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

pub mod bot;
pub mod export;
pub mod list;
pub mod login;
pub mod oauth;
pub mod run;

use anyhow::{Context, Result};
use awb_domain::i18n::Catalog;
use awb_domain::profile::AuthMethod;
use awb_mw_api::client::{MediaWikiClient, ReqwestMwClient};
use awb_security::{CredentialPort, InMemoryCredentialStore};
use awb_storage::TomlConfigStore;
use console::style;
use std::path::Path;
use url::Url;

pub(crate) fn emit_stdout_line(line: &str) {
    use std::io::Write;
    let mut out = std::io::stdout().lock();
    let _ = out.write_all(line.as_bytes());
    let _ = out.write_all(b"\n");
}

pub(crate) fn bot_password_username<'a>(
    auth_method: &'a AuthMethod,
    cat: &Catalog,
) -> Result<&'a str> {
    match auth_method {
        AuthMethod::BotPassword { username } => Ok(username.as_str()),
        AuthMethod::OAuth2 { .. } => anyhow::bail!("{}", cat.err_oauth2_unimplemented),
        AuthMethod::OAuth1 { .. } => anyhow::bail!("{}", cat.err_oauth1_unimplemented),
    }
}

pub(crate) async fn authenticate_client<C: MediaWikiClient>(
    client: &C,
    auth_method: &AuthMethod,
    password: &str,
    cat: &Catalog,
) -> Result<()> {
    print!("{}", cat.msg_logging_in);
    let username = bot_password_username(auth_method, cat)?;

    client
        .login_bot_password(username, password)
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

    Ok(())
}

pub(crate) async fn connect_authenticated_client(
    wiki: &Url,
    profile_path: &Path,
    auth_profile: &str,
    cat: &Catalog,
) -> Result<(ReqwestMwClient, String)> {
    // Load profile
    let config_store = TomlConfigStore::new(profile_path);
    let profile = config_store
        .load_profile(auth_profile)
        .context(cat.err_load_profile)?;

    // Get credentials
    let cred_store = InMemoryCredentialStore::new();
    let password = cred_store
        .get_password(auth_profile)
        .context(cat.err_no_stored_creds)?;

    // Create client and login
    let client = ReqwestMwClient::new(wiki.clone(), profile.throttle_policy.clone())
        .context(cat.err_create_http_client)?;

    authenticate_client(&client, &profile.auth_method, &password, cat).await?;

    Ok((client, password))
}

#[cfg(test)]
pub(crate) mod test_support {
    use async_trait::async_trait;
    use awb_domain::profile::{AuthMethod, Profile, ThrottlePolicy};
    use awb_domain::types::{
        Namespace, PageContent, PageId, PageProperties, ProtectionInfo, RevisionId, Title,
    };
    use awb_mw_api::client::{EditRequest, EditResponse, MediaWikiClient};
    use awb_mw_api::error::MwApiError;
    use awb_mw_api::oauth::{OAuth1Config, OAuthSession};
    use awb_storage::TomlConfigStore;
    use chrono::Utc;
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use url::Url;

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    pub(crate) struct TestTempDir(PathBuf);

    impl TestTempDir {
        pub(crate) fn new(prefix: &str) -> Self {
            let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!(
                "awb-cli-test-{prefix}-{}-{seq}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).expect("create temp dir");
            Self(dir)
        }

        pub(crate) fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestTempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    pub(crate) fn write_test_profile(path: &Path, id: &str, wiki: &Url) {
        let store = TomlConfigStore::new(path);
        let profile = Profile {
            id: id.to_string(),
            name: "Test Bot".to_string(),
            api_url: wiki.clone(),
            auth_method: AuthMethod::BotPassword {
                username: "TestBot@bot".to_string(),
            },
            default_namespaces: HashSet::new(),
            throttle_policy: ThrottlePolicy::default(),
        };
        store.save_profile(&profile).expect("save test profile");
    }

    pub(crate) struct MockMwClient {
        pages: HashMap<String, PageContent>,
        pub(crate) fail_login: bool,
        pub(crate) fail_csrf: bool,
        pub(crate) fail_edit: bool,
    }

    impl MockMwClient {
        pub(crate) fn new() -> Self {
            Self {
                pages: HashMap::new(),
                fail_login: false,
                fail_csrf: false,
                fail_edit: false,
            }
        }

        pub(crate) fn add_page(&mut self, title: &str, wikitext: &str) {
            let page = PageContent {
                page_id: PageId(1),
                title: Title::new(Namespace::MAIN, title),
                revision: RevisionId(100),
                timestamp: Utc::now(),
                wikitext: wikitext.to_string(),
                size_bytes: wikitext.len() as u64,
                is_redirect: false,
                protection: ProtectionInfo::default(),
                properties: PageProperties::default(),
            };
            self.pages.insert(title.to_string(), page);
        }
    }

    #[async_trait]
    impl MediaWikiClient for MockMwClient {
        async fn login_bot_password(
            &self,
            _username: &str,
            _password: &str,
        ) -> Result<(), MwApiError> {
            if self.fail_login {
                return Err(MwApiError::ApiError {
                    code: "loginfailed".to_string(),
                    info: "bad credentials".to_string(),
                });
            }
            Ok(())
        }

        async fn login_oauth1(&self, _config: OAuth1Config) -> Result<(), MwApiError> {
            Ok(())
        }

        async fn login_oauth2(&self, _session: OAuthSession) -> Result<(), MwApiError> {
            Ok(())
        }

        async fn fetch_csrf_token(&self) -> Result<String, MwApiError> {
            if self.fail_csrf {
                return Err(MwApiError::ApiError {
                    code: "csrffailed".to_string(),
                    info: "token unavailable".to_string(),
                });
            }
            Ok("mock+\\".to_string())
        }

        async fn get_page(&self, title: &Title) -> Result<PageContent, MwApiError> {
            self.pages
                .get(&title.display)
                .cloned()
                .ok_or_else(|| MwApiError::ApiError {
                    code: "missingtitle".to_string(),
                    info: "page not found".to_string(),
                })
        }

        async fn edit_page(&self, _edit: &EditRequest) -> Result<EditResponse, MwApiError> {
            if self.fail_edit {
                return Err(MwApiError::ApiError {
                    code: "protectedpage".to_string(),
                    info: "cannot edit".to_string(),
                });
            }
            Ok(EditResponse {
                result: "Success".to_string(),
                new_revid: Some(4242),
                new_timestamp: Some(Utc::now().to_rfc3339()),
            })
        }

        async fn parse_wikitext(
            &self,
            _wikitext: &str,
            _title: &Title,
        ) -> Result<String, MwApiError> {
            Ok("<p>ok</p>".to_string())
        }

        async fn list_category_members(
            &self,
            _category: &str,
            _limit: u32,
        ) -> Result<Vec<String>, MwApiError> {
            Ok(vec![])
        }

        async fn search_pages(&self, _query: &str, _limit: u32) -> Result<Vec<String>, MwApiError> {
            Ok(vec![])
        }

        async fn get_backlinks(
            &self,
            _title: &str,
            _limit: u32,
        ) -> Result<Vec<String>, MwApiError> {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use awb_domain::i18n::Locale;
    use secrecy::SecretString;

    #[test]
    fn test_bot_password_username_variants() {
        let cat = Locale::De.catalog();
        let bot_auth = AuthMethod::BotPassword {
            username: "WikiBot@main".to_string(),
        };
        assert_eq!(
            bot_password_username(&bot_auth, cat).unwrap(),
            "WikiBot@main"
        );

        let oauth1 = AuthMethod::OAuth1 {
            consumer_key: "ck".to_string(),
            consumer_secret: SecretString::new("cs".to_string().into()),
            access_token: "at".to_string(),
            access_secret: SecretString::new("as".to_string().into()),
        };
        let err1 = bot_password_username(&oauth1, cat).unwrap_err();
        assert!(err1.to_string().contains(cat.err_oauth1_unimplemented));

        let oauth2 = AuthMethod::OAuth2 {
            client_id: "cid".to_string(),
            client_secret: SecretString::new("csec".to_string().into()),
        };
        let err2 = bot_password_username(&oauth2, cat).unwrap_err();
        assert!(err2.to_string().contains(cat.err_oauth2_unimplemented));
    }

    #[tokio::test]
    async fn test_authenticate_client_success_and_failures() {
        let cat = Locale::De.catalog();
        let auth = AuthMethod::BotPassword {
            username: "Bot@main".to_string(),
        };

        let ok_client = test_support::MockMwClient::new();
        authenticate_client(&ok_client, &auth, "pw", cat)
            .await
            .expect("auth succeeds");

        let mut fail_login = test_support::MockMwClient::new();
        fail_login.fail_login = true;
        let err = authenticate_client(&fail_login, &auth, "pw", cat)
            .await
            .unwrap_err();
        assert!(err.to_string().contains(cat.err_login_failed));

        let mut fail_csrf = test_support::MockMwClient::new();
        fail_csrf.fail_csrf = true;
        let err = authenticate_client(&fail_csrf, &auth, "pw", cat)
            .await
            .unwrap_err();
        assert!(err.to_string().contains(cat.err_fetch_csrf));
    }

    #[tokio::test]
    async fn test_connect_authenticated_client_missing_profile_and_creds() {
        let tmp = test_support::TestTempDir::new("connect");
        let wiki = Url::parse("https://en.wikipedia.org/w/api.php").unwrap();
        let cat = Locale::Fr.catalog();

        let missing_path = tmp.path().join("missing.toml");
        let err = connect_authenticated_client(&wiki, &missing_path, "default", cat)
            .await
            .err()
            .expect("expected missing profile error");
        assert!(err.to_string().contains(cat.err_load_profile));

        let valid_path = tmp.path().join("config.toml");
        test_support::write_test_profile(&valid_path, "default", &wiki);
        let err = connect_authenticated_client(&wiki, &valid_path, "default", cat)
            .await
            .err()
            .expect("expected missing credentials error");
        assert!(err.to_string().contains(cat.err_no_stored_creds));
    }
}
