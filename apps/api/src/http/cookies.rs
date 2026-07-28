use axum::http::{HeaderMap, HeaderValue, header};
use cookie::{Cookie, SameSite};
use uuid::Uuid;

use crate::config::Config;

pub const SESSION_COOKIE: &str = "mailforge_session";
pub const OAUTH_FLOW_COOKIE: &str = "mailforge_oauth_flow";

pub struct CookieManager {
    config: Config,
}

impl CookieManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn session_cookie(&self, session_id: &Uuid) -> String {
        self.build_cookie(
            SESSION_COOKIE,
            &session_id.to_string(),
            "/",
            self.config.session_ttl_seconds,
        )
    }

    pub fn oauth_flow_cookie(&self, state: &str, verifier: &str) -> String {
        self.build_cookie(
            OAUTH_FLOW_COOKIE,
            &format!("{state}.{verifier}"),
            "/auth/google",
            600,
        )
    }

    pub fn expired_oauth_flow_cookie(&self) -> String {
        self.build_cookie(OAUTH_FLOW_COOKIE, "", "/auth/google", 0)
    }

    pub fn session_id(&self, headers: &HeaderMap) -> Option<Uuid> {
        self.cookie_value(headers, SESSION_COOKIE)
            .and_then(|value| Uuid::parse_str(&value).ok())
    }

    pub fn oauth_flow(&self, headers: &HeaderMap) -> Option<(String, String)> {
        self.cookie_value(headers, OAUTH_FLOW_COOKIE)?
            .split_once('.')
            .map(|(state, verifier)| (state.to_string(), verifier.to_string()))
    }

    fn cookie_value(&self, headers: &HeaderMap, name: &str) -> Option<String> {
        let cookies = headers.get(header::COOKIE)?.to_str().ok()?;
        Cookie::split_parse(cookies).find_map(|parsed_cookie| {
            let cookie = parsed_cookie.ok()?;
            (cookie.name() == name).then_some(cookie.value().to_owned())
        })
    }

    fn build_cookie(&self, name: &str, value: &str, path: &str, max_age: u64) -> String {
        let max_age = cookie::time::Duration::seconds(max_age.min(i64::MAX as u64) as i64);

        Cookie::build((name, value))
            .path(path)
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(self.config.cookie_secure)
            .max_age(max_age)
            .build()
            .to_string()
    }
}

pub fn cookie_header(value: &str) -> HeaderValue {
    HeaderValue::from_str(value).expect("cookie value must be a valid header")
}
