use serde::Deserialize;
use tokio::fs;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub url: Url,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwtKey {
    pub key: String,
    pub kid: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub jwt_key: JwtKey,
}

pub(crate) async fn read_config(cfg_path: &str) -> Config {
    match fs::read_to_string(cfg_path).await {
        Ok(x) => toml::from_str(&x).expect("Invalid config file"),
        Err(_) => panic!("Config file not found!"),
    }
}
