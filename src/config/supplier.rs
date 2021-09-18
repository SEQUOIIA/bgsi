use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Supplier {
    pub name : String,
    pub secret : String,
    pub receivers : Vec<String>
}