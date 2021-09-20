use std::io::Read;
use crate::model::{STResult, STError};
use crate::config::Data;
use std::sync::Arc;
use crate::config::receiver::Provider;
use std::collections::HashMap;
use crate::config::rule::{Action, Repo};
use crate::model::github_push::GitHubPushPayload;

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

pub fn handle(secret : String, data : Arc<Data>, webhook_payload : Vec<u8>) -> STResult<()> {
    let supplier = data.get_supplier_by_secret(&secret)?;
    let rules = data.get_rules_by_supplier_name(&supplier.name)?;

    // Rule engine (of sorts)
    let mut accounts : HashMap<String, Action> = HashMap::new();
    let mut repositories : HashMap<String, Repo> = HashMap::new();

    for rule in &rules {
        for acc in &rule.accounts {
            accounts.insert(acc.to_owned(), rule.action.clone());
        }

        for repo in &rule.repos {
            let split : Vec<&str> = repo.split('/').collect();
            if split.len() == 2 {
                let acc_name = split[0];
                let repo_name = split[1];
                repositories.insert(repo.to_owned(), Repo {
                    account: acc_name.to_owned(),
                    repo: repo_name.to_owned(),
                    action: rule.action.clone()
                });
            }
        }
    }

    let push_resp = load_github_push_payload(webhook_payload)?;

    println!("{:?}", accounts);
    println!("{:?}", repositories);

    for receiver_name in supplier.receivers {
        let receiver = data.get_receiver_by_name(&receiver_name)?;
        match receiver.provider {
            Provider::Slack(_) => {}
            Provider::CustomWebhook => {}
        }
    }

    Ok(())
}

fn load_github_push_payload(payload : Vec<u8>) -> STResult<GitHubPushPayload> {
    return match serde_json::from_slice(&payload) {
        Ok(val) => Ok(val),
        Err(err) => Err(STError::JsonError(err))
    }
}

fn handle_slack() {

}