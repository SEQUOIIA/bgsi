use std::io::Read;
use crate::model::{STResult, STError};
use crate::config::Data;
use std::sync::Arc;
use crate::config::receiver::Provider;
use std::collections::HashMap;
use crate::config::rule::{Action, Repo};
use crate::model::github_push::{GitHubPushPayload, GitRef};

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

#[derive(Clone, Default, Debug)]
pub struct HandleResult {
    pub msg : String,
    pub accounts : HashMap<String, Action>,
    pub repositories : HashMap<String, Repo>
}

pub fn handle(secret : String, data : Arc<Data>, webhook_payload : Vec<u8>) -> STResult<HandleResult> {
    let supplier = data.get_supplier_by_secret(&secret)?;
    let mut rules = data.get_rules_by_supplier_name(&supplier.name)?;

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
    let msg = create_message_from_gpp(&push_resp)?;

    for receiver_name in supplier.receivers {
        let receiver = data.get_receiver_by_name(&receiver_name)?;
        match receiver.provider {
            Provider::Slack(_) => {}
            Provider::CustomWebhook => {}
            Provider::Testing => {
                println!("{}", &receiver_name);
                println!("{}", msg);

                println!("{:?}", accounts);
                println!("{:?}", repositories);
            }
        }
    }

    Ok(HandleResult {
        msg,
        accounts,
        repositories
    })
}

fn load_github_push_payload(payload : Vec<u8>) -> STResult<GitHubPushPayload> {
    return match serde_json::from_slice(&payload) {
        Ok(val) => Ok(val),
        Err(err) => Err(STError::JsonError(err))
    }
}

fn create_message_from_gpp(gpp : &GitHubPushPayload) -> STResult<String> {
    let push_type = GitRef::from_str(&gpp.ref_field)?;
    let mut msg = String::new();
    let mut action = "new".to_owned();
    {
        if gpp.created {
            action = "new".to_owned();
        }
        if gpp.deleted {
            action = "removed".to_owned();
        }
    }

    match push_type {
        GitRef::Branch(branch) => {

            if gpp.commits.len() == 0 {
                if gpp.created {
                    action = "created".to_owned();
                }
                msg = format!("[{}] branch '{}' {} by {}", gpp.repository.full_name, branch, action, gpp.sender.login);
            } else {
                let multiple_commits_msg = if gpp.commits.len() > 1 { "s" } else { "" };
                if gpp.deleted {
                    action = "deleted".to_owned();
                }
                msg = format!("[{}] {} {} commit{} pushed to {} by {}", gpp.repository.full_name,  gpp.commits.len(), action, multiple_commits_msg, branch, gpp.sender.login);
            }
        }
        GitRef::Tag(tag) => {
            msg = format!("[{}] Tag '{}' {} by {}", gpp.repository.full_name, tag, action, gpp.sender.login);
        }
    }

    Ok(msg)
}

fn handle_slack() {

}