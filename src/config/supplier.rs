use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Supplier {
    pub name : String,
    pub secret : String,
    receivers : Vec<String>
}