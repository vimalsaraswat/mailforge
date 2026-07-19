use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let host = env::var("HOST").expect("Missing required environment variable: HOST");

        let port = env::var("PORT")
            .expect("Missing required environment variable: PORT")
            .parse::<u16>()
            .expect("PORT must be a valid u16");

        Self { host, port }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
