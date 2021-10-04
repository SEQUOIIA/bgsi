use crate::config::provider::SlackData;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::config::receiver::Provider::{Slack, CustomWebhook};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Receiver {
    pub name : String,
    pub provider : Provider
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Provider {
    Slack(SlackData),
    CustomWebhook,
    Testing
}