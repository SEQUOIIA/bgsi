use crate::config::provider::SlackData;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Receiver {
    pub name : String,
    pub provider : Provider
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Provider {
    Slack(SlackData),
    CustomWebhook
}

impl Display for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            slack => f.write_str("slack"),
            custom_webhook => f.write_str("custom_webhook")
        }
    }
}