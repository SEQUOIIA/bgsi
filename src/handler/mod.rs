use std::io::Read;
use crate::model::STResult;
use crate::config::Data;
use std::sync::Arc;
use crate::config::receiver::Provider;

pub struct GitHubWebhook {
}

impl GitHubWebhook {
    pub fn new() -> Self {
        Self {
        }
    }
}

pub fn handle(secret : String, data : Arc<Data>) -> STResult<()> {
    let supplier = data.get_supplier_by_secret(&secret)?;

    for receiver_name in supplier.receivers {
        let receiver = data.get_receiver_by_name(&receiver_name)?;
        match receiver.provider {
            Provider::Slack(_) => {}
            Provider::CustomWebhook => {}
        }
    }

    Ok(())
}