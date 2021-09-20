use std::io::Read;
use crate::model::STResult;
use crate::config::Data;
use std::sync::Arc;
use crate::config::receiver::Provider;
use std::collections::HashMap;
use crate::config::rule::Action;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::data::*;

    #[test]
    fn handle_load_data() {
        let data = Data::new_from_bytes(RULES_TEST_DATA.as_bytes(), RECEIVERS_TEST_DATA.as_bytes(), SUPPLIERS_TEST_DATA.as_bytes());
        assert!(data.suppliers.len() > 0);
        assert!(data.receivers.len() > 0);
        assert!(data.rules.len() > 0);
    }
}

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
    let rules = data.get_rules_by_supplier_name(&supplier.name)?;

    // Rule engine (of sorts)
    let accounts : HashMap<String, Action> = HashMap::new();
    let repositories : HashMap<String, Action> = HashMap::new();

    for receiver_name in supplier.receivers {
        let receiver = data.get_receiver_by_name(&receiver_name)?;
        match receiver.provider {
            Provider::Slack(_) => {}
            Provider::CustomWebhook => {}
        }
    }

    Ok(())
}

fn handle_slack() {

}