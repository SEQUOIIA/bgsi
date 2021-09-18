use serde::{Deserialize, Serialize};
use std::io::Read;
use crate::model::STResult;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use crate::config::rule::Rule;
use crate::config::supplier::Supplier;
use crate::config::receiver::Receiver;

mod receiver;
mod rule;
mod supplier;
mod provider;

const ENV_PREFIX : &str = "BGSI_";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(rename = "port", default = "default_port")]
    pub port : String,
    #[serde(default = "default_address")]
    pub address : String,
    #[serde(default = "default_data")]
    pub data : Data
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: default_port(),
            address: default_address(),
            data: default_data()
        }
    }
}

impl Config {
    pub fn load() -> Self {
        load_conf_from_yaml("config.yml").unwrap_or_default()
    }

    pub fn load_rules() -> STResult<Vec<Rule>> {
        load_conf_from_yaml("rules.yml")
    }

    pub fn load_receivers() -> STResult<Vec<Receiver>> {
        load_conf_from_yaml("receivers.yml")
    }

    pub fn load_suppliers() -> STResult<Vec<Supplier>> {
        load_conf_from_yaml("suppliers.yml")
    }

    fn load_envs(&mut self) {
        if let Some(val) = Self::retrieve_env("port") {
            self.port = val;
        }
        if let Some(val) = Self::retrieve_env("address") {
            self.address = val;
        }
    }

    fn retrieve_env(val : &str) -> Option<String> {
        match std::env::var(val.to_uppercase()) {
            Ok(env_value) => Some(env_value),
            _Err => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Data {
    pub rules : HashMap<String, Rule>,
    pub receivers : HashMap<String, Receiver>,
    pub suppliers : HashMap<String, Supplier>,
}

impl Data {
    pub fn new() -> Self {
        let rules = load_conf_from_yaml::<Vec<Rule>>("rules.yml").unwrap().into_iter()
            .map(|item| {
                (item.name.clone(), item)
            }).collect();

        let receivers = load_conf_from_yaml::<Vec<Receiver>>("receivers.yml").unwrap().into_iter()
            .map(|item| {
                (item.name.clone(), item)
            }).collect();

        let suppliers = load_conf_from_yaml::<Vec<Supplier>>("suppliers.yml").unwrap().into_iter()
            .map(|item| {
                (item.name.clone(), item)
            }).collect();

        Self {
            rules,
            receivers,
            suppliers
        }
    }
}

fn load_conf_from_yaml<T>(path : &str) -> STResult<T> where T : DeserializeOwned {
    let mut input_file = std::fs::File::open(path)?;
    let mut buf = Vec::new();
    input_file.read_to_end(&mut buf)?;
    Ok(serde_yaml::from_slice::<T>(&buf)?)
}

fn default_port() -> String {
    "8080".to_owned()
}

fn default_address() -> String {
    "0.0.0.0".to_owned()
}

fn default_data() -> Data {
    Data::new()
}