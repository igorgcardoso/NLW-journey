#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub api_base_url: String,
    pub web_base_url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Config {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is required"),
            api_base_url: std::env::var("API_BASE_URL").expect("API_BASE_URL is required"),
            web_base_url: std::env::var("WEB_BASE_URL").expect("WEB_BASE_URL is required"),
            port: std::env::var("PORT")
                .unwrap_or("3333".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
