use std::time::Duration;

use chrono::Utc;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
    basic::BasicClient, url,
};

use crate::clients::google::models::{GoogleToken, GoogleUserInfo};

type GoogleClient = BasicClient<
    EndpointSet,    // Auth URL
    EndpointNotSet, // Device Auth URL
    EndpointNotSet, // Introspection URL
    EndpointNotSet, // Revocation URL
    EndpointSet,    // Token URL
>;

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";

#[derive(Clone)]
pub struct GoogleOAuthClient {
    client: GoogleClient,
    http: reqwest::Client,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let http = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");

        let client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(AuthUrl::new(AUTH_URL.to_string()).expect("invalid auth url"))
            .set_token_uri(TokenUrl::new(TOKEN_URL.to_string()).expect("invalid token url"))
            .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("invalid redirect uri"));

        Self { client, http }
    }

    pub fn authorization_url(
        &self,
        include_gmail: bool,
    ) -> (url::Url, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let mut auth_request = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".into()))
            .add_scope(Scope::new("email".into()))
            .add_scope(Scope::new("profile".into()));

        if include_gmail {
            auth_request = auth_request
                .add_scope(Scope::new(
                    "https://www.googleapis.com/auth/gmail.send".into(),
                ))
                .add_extra_param("prompt", "consent")
                .add_extra_param("access_type", "offline");
        }

        let (url, csrf) = auth_request.set_pkce_challenge(pkce_challenge).url();

        (url, csrf, pkce_verifier)
    }

    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
    ) -> anyhow::Result<GoogleToken> {
        let token = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.http)
            .await?;

        Ok(Self::to_google_token(&token))
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: RefreshToken,
    ) -> anyhow::Result<GoogleToken> {
        let token = self
            .client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.http)
            .await?;

        Ok(Self::to_google_token(&token))
    }

    pub async fn user_info(&self, access_token: &str) -> anyhow::Result<GoogleUserInfo> {
        Ok(self
            .http
            .get(USERINFO_URL)
            .bearer_auth(access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<GoogleUserInfo>()
            .await?)
    }

    fn to_google_token<T: TokenResponse>(token: &T) -> GoogleToken {
        GoogleToken {
            access_token: token.access_token().secret().clone(),
            refresh_token: token.refresh_token().map(|t| t.secret().clone()),
            expires_at: token
                .expires_in()
                .and_then(|d| chrono::Duration::from_std(d).ok())
                .map(|d| Utc::now() + d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_oauth_client_initialization() {
        let client = GoogleOAuthClient::new(
            "test_client_id".into(),
            "test_client_secret".into(),
            "http://localhost:3000/auth/callback".into(),
        );

        let (url, csrf, _verifier) = client.authorization_url(false);
        assert!(url.as_str().starts_with(AUTH_URL));
        assert!(!csrf.secret().is_empty());
        assert!(!url.as_str().contains("gmail.send"));

        let (gmail_url, _, _) = client.authorization_url(true);
        assert!(gmail_url.as_str().contains("gmail.send"));
        assert!(gmail_url.as_str().contains("access_type=offline"));
        assert!(gmail_url.as_str().contains("prompt=consent"));
    }
}
