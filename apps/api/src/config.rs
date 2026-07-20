use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
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

        Self {
            host,
            port,
            database_url,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
