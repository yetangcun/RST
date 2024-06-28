use serde::Deserialize;

#[derive(Deserialize)]
pub struct Cfg {
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    pub mysql_url: String,
}