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
            Some(self.config.session_ttl_seconds),
        )
    }

    pub fn expired_session_cookie(&self) -> String {
        self.build_cookie(SESSION_COOKIE, "", "/", None)
    }

    pub fn oauth_flow_cookie(&self, state: &str, verifier: &str, connect: bool) -> String {
        self.build_cookie(
            OAUTH_FLOW_COOKIE,
            &format!("{state}.{verifier}.{connect}"),
            "/auth/google",
            Some(600),
        )
    }

    pub fn expired_oauth_flow_cookie(&self) -> String {
        self.build_cookie(OAUTH_FLOW_COOKIE, "", "/auth/google", None)
    }

    pub fn session_id(&self, headers: &HeaderMap) -> Option<Uuid> {
        self.cookie_value(headers, SESSION_COOKIE)
            .and_then(|value| Uuid::parse_str(&value).ok())
    }

    pub fn oauth_flow(&self, headers: &HeaderMap) -> Option<(String, String, bool)> {
        let val = self.cookie_value(headers, OAUTH_FLOW_COOKIE)?;
        let mut parts = val.splitn(3, '.');
        let state = parts.next()?.to_string();
        let verifier = parts.next()?.to_string();
        let connect = parts.next()?.parse::<bool>().unwrap_or(false);
        Some((state, verifier, connect))
    }

    fn cookie_value(&self, headers: &HeaderMap, name: &str) -> Option<String> {
        let cookies = headers.get(header::COOKIE)?.to_str().ok()?;
        Cookie::split_parse(cookies).find_map(|parsed_cookie| {
            let cookie = parsed_cookie.ok()?;
            (cookie.name() == name).then_some(cookie.value().to_owned())
        })
    }

    fn build_cookie(&self, name: &str, value: &str, path: &str, max_age: Option<u64>) -> String {
        let mut builder = Cookie::build((name, value))
            .path(path)
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(self.config.cookie_secure);

        if let Some(age) = max_age {
            let duration = cookie::time::Duration::seconds(age.min(i64::MAX as u64) as i64);
            builder = builder.max_age(duration);
        } else {
            builder = builder.max_age(cookie::time::Duration::seconds(0));
        }

        builder.build().to_string()
    }
}

pub fn cookie_header(value: &str) -> HeaderValue {
    HeaderValue::from_str(value).expect("cookie value must be a valid header")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 3000,
            database_url: "postgres://localhost/test".into(),
            google_client_id: "id".into(),
            google_client_secret: "sec".into(),
            google_redirect_uri: "http://localhost/callback".into(),
            frontend_url: "http://localhost:3001".into(),
            session_ttl_seconds: 3600,
            cookie_secure: false,
        }
    }

    #[test]
    fn test_session_id_extraction() {
        let manager = CookieManager::new(test_config());
        let session_id = Uuid::new_v4();
        let cookie_str = format!("mailforge_session={}", session_id);

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, HeaderValue::from_str(&cookie_str).unwrap());

        assert_eq!(manager.session_id(&headers), Some(session_id));
    }

    #[test]
    fn test_oauth_flow_extraction() {
        let manager = CookieManager::new(test_config());
        let cookie_str = "mailforge_oauth_flow=my_state.my_verifier.true";

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, HeaderValue::from_str(cookie_str).unwrap());

        let flow = manager.oauth_flow(&headers);
        assert_eq!(
            flow,
            Some(("my_state".into(), "my_verifier".into(), true))
        );
    }
}
