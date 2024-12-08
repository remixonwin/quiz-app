use quiz_app_backend::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from config
    let _config = config::get_config()?;

    // Add your seeding logic here
    println!("Seeding database...");

    Ok(())
}
