use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rule {
    pub name : String,
    supplier : String,
    pub action : Action,
    accounts : Vec<String>,
    repos: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Allow,
    Deny
}