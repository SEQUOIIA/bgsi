use serde::{Deserialize, Serialize};
use std::io::Read;
use crate::model::STResult;
use std::collections::HashMap;
use serde::de::DeserializeOwned;

const ENV_PREFIX : &str = "BGSI_";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(rename = "port", default = "default_port")]
    pub port : String,
    #[serde(default = "default_address")]
    pub address : String,
    pub github_webhook_secret : String,
}

impl Config {
    pub fn load() -> STResult<Self> {
        let mut input_file = std::fs::File::open("config.yml")?;
        let mut buf = Vec::new();
        input_file.read_to_end(&mut buf)?;
        Ok(serde_yaml::from_slice::<Config>(&buf)?)
    }

    fn load_envs(&mut self) {
        if let Some(val) = Self::retrieve_env("port") {
            self.port = val;
        }
        if let Some(val) = Self::retrieve_env("address") {
            self.address = val;
        }
        if let Some(val) = Self::retrieve_env("github_webhook_secret") {
            self.github_webhook_secret = val;
        }
    }

    fn retrieve_env(val : &str) -> Option<String> {
        match std::env::var(val.to_uppercase()) {
            Ok(env_value) => Some(env_value),
            _Err => None
        }
    }
}


fn default_port() -> String {
    "8080".to_owned()
}

fn default_address() -> String {
    "0.0.0.0".to_owned()
}
