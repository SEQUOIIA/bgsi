use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Rule {
    pub name : String,
    pub supplier : String,
    pub action : Action,
    #[serde(default = "default_vec_string")]
    pub accounts : Vec<String>,
    #[serde(default = "default_vec_string")]
    pub repos: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Allow,
    Deny
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Repo {
    pub account : String,
    pub repo : String,
    pub action : Action
}

fn default_vec_string() -> Vec<String> {
    Vec::new()
}