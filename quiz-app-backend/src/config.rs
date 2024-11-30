use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

pub fn get_config() -> Result<Config, env::VarError> {
    Ok(Config {
        database_url: get_database_url(),
        jwt_secret: get_jwt_secret(),
    })
}

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://remixonwin:600181@localhost/quiz_app".to_string()
    })
}

pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        "your-256-bit-secret".to_string()
    })
}
