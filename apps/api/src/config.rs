use std::{env, path::Path};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub frontend_url: String,
    pub session_ttl_seconds: u64,
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> Self {
        // Load the API-local file even when the binary is started from the
        // repository root. Already-exported environment variables still win.
        dotenvy::from_path(Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
        dotenvy::dotenv().ok();

        let get_env = |key: &str| {
            env::var(key)
                .unwrap_or_else(|_| panic!("Missing required environment variable: {}", key))
        };
        let get_env_default =
            |key: &str, default: &str| env::var(key).unwrap_or_else(|_| default.to_string());

        Self {
            host: get_env("HOST"),
            port: get_env("PORT").parse().expect("PORT must be a valid u16"),
            database_url: get_env("DATABASE_URL"),
            google_client_id: get_env("GOOGLE_CLIENT_ID"),
            google_client_secret: get_env("GOOGLE_CLIENT_SECRET"),
            google_redirect_uri: get_env_default(
                "GOOGLE_REDIRECT_URI",
                "http://127.0.0.1:3000/auth/google/callback",
            ),
            frontend_url: get_env_default("FRONTEND_URL", "http://localhost:3001"),
            session_ttl_seconds: get_env_default("SESSION_TTL_SECONDS", "604800")
                .parse()
                .expect("SESSION_TTL_SECONDS must be a valid number"),
            cookie_secure: get_env_default("COOKIE_SECURE", "false")
                .parse()
                .expect("COOKIE_SECURE must be true or false"),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
