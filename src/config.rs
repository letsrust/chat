use std::fs::File;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from app.yml
        let ret = match File::open("app.yml") {
            Ok(file) => {
                let config: AppConfig = serde_yaml::from_reader(file)?;
                config
            }
            Err(_) => bail!("config file not found"),
        };

        Ok(ret)
    }
}
