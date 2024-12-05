use std::env;
use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub workers: usize,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        Ok(Config {
            jwt_secret: env::var("JWT_SECRET").map_err(|_| AppError::ConfigError("JWT_SECRET not set".to_string()))?,
            database_url: env::var("DATABASE_URL").map_err(|_| AppError::ConfigError("DATABASE_URL not set".to_string()))?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            workers: env::var("WORKERS")
                .unwrap_or_else(|_| num_cpus::get().to_string())
                .parse()
                .unwrap_or(num_cpus::get()),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        })
    }
}

pub fn get_config() -> Result<Config, env::VarError> {
    Config::new()
}

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://remixonwin:600181@localhost/quiz_app".to_string())
}

pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        "super_secure_jwt_secret_key".to_string() // Ensure a strong default
    })
}
