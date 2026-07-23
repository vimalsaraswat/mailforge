use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenUrl, basic::BasicClient, url,
};

type GoogleClient = BasicClient<
    EndpointSet,    // Auth URL
    EndpointNotSet, // Device Auth URL
    EndpointNotSet, // Introspection URL
    EndpointNotSet, // Revocation URL
    EndpointSet,    // Token URL
>;

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub struct GoogleOAuthClient {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,

    http: reqwest::Client,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let http = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("failed to build reqwest client");

        Self {
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            redirect_uri: RedirectUrl::new(redirect_uri).expect("invalid redirect uri"),
            http,
        }
    }

    fn client(&self) -> GoogleClient {
        BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(AuthUrl::new(AUTH_URL.to_string()).expect("invalid auth url"))
            .set_token_uri(TokenUrl::new(TOKEN_URL.to_string()).expect("invalid token url"))
            .set_redirect_uri(self.redirect_uri.clone())
    }

    pub fn authorization_url(&self) -> (url::Url, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (url, csrf) = self
            .client()
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".into()))
            .add_scope(Scope::new("email".into()))
            .add_scope(Scope::new("profile".into()))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/gmail.send".into(),
            ))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (url, csrf, pkce_verifier)
    }
}
