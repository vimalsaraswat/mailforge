use std::env;
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
        dotenvy::dotenv().ok();

        let host = env::var("HOST").expect("Missing required environment variable: HOST");

        let port = env::var("PORT")
            .expect("Missing required environment variable: PORT")
            .parse::<u16>()
            .expect("PORT must be a valid u16");

        let database_url =
            env::var("DATABASE_URL").expect("Missing required environment variable: DATABASE_URL");

        let session_ttl_seconds = env::var("SESSION_TTL_SECONDS")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<u64>()
            .expect("SESSION_TTL_SECONDS must be a valid number");

        let cookie_secure = env::var("COOKIE_SECURE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("COOKIE_SECURE must be true or false");

        Self {
            host,
            port,
            database_url,
            google_client_id: env::var("GOOGLE_CLIENT_ID")
                .expect("Missing required environment variable: GOOGLE_CLIENT_ID"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("Missing required environment variable: GOOGLE_CLIENT_SECRET"),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/google/callback".to_string()),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            session_ttl_seconds,
            cookie_secure,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
