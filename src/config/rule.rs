use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Rule {
    pub name : String,
    pub supplier : String,
    pub action : Action,
    accounts : Vec<String>,
    repos: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Allow,
    Deny
}