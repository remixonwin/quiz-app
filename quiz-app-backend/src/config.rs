use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

#[allow(dead_code)]
pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    Ok(Config {
        database_url: std::env::var("DATABASE_URL")?,
        jwt_secret: std::env::var("JWT_SECRET")?,
        port: std::env::var("PORT")?.parse()?,
    })
}

#[allow(dead_code)]
pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://remixonwin:600181@localhost/quiz_app".to_string()
    })
}

#[allow(dead_code)]
pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        "your-256-bit-secret".to_string()
    })
}
