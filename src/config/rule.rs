use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Rule {
    pub name : String,
    pub priority: Option<i32>,
    pub supplier : String,
    pub action : Action,
    #[serde(default = "default_vec_string")]
    pub accounts : Vec<String>,
    #[serde(default = "default_vec_string")]
    pub repos: Vec<String>
}

impl Eq for Rule {}

impl PartialEq<Self> for Rule {
    fn eq(&self, other: &Self) -> bool {
        let main = match self.priority {
            Some(val) => val,
            None => 1
        };
        let second = match other.priority {
            Some(val) => val,
            None => 1
        };

        main == second
    }
}

impl PartialOrd<Self> for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let main = match self.priority {
            Some(val) => val,
            None => 1
        };
        let second = match other.priority {
            Some(val) => val,
            None => 1
        };

        Some(main.cmp(&second))
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        let main = match self.priority {
            Some(val) => val,
            None => 1
        };
        let second = match other.priority {
            Some(val) => val,
            None => 1
        };

        main.cmp(&second)
    }
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