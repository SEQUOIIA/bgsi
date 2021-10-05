use serde::{Deserialize, Serialize};
use crate::model::github_push::{GitHubPushPayload, GitRef};
use crate::model::STResult;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlackData {
    pub webhook_url : String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlackReceiver {
    pub data : SlackData
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlackRequestBody {
    pub text : String
}

impl SlackReceiver {
    pub fn handle(msg : String, gpp : &GitHubPushPayload) {


        let req = slack::SlackRequestBody {
            blocks: vec![slack::Block {
                type_field: "section".to_owned(),
                text: Some(slack::Text {
                    type_field: "mrkdwn".to_owned(),
                    text: Self::create_message_from_gpp(&gpp).unwrap()
                })
            },
            slack::Block {
                type_field: "divider".to_owned(),
                text: None
            },
            slack::Block {
                type_field: "section".to_owned(),
                text: Some(slack::Text {
                    type_field: "mrkdwn".to_owned(),
                    text: "1ded7f5` - init commit\n`1ded7f5` - Created README.md".to_owned()
                })
            }
            ]
        };
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
                    msg = format!("<{}|[{}]> branch '{}' {} by {}", gpp.repository.url, gpp.repository.full_name, branch, action, gpp.sender.login);
                } else {
                    let multiple_commits_msg = if gpp.commits.len() > 1 { "s" } else { "" };
                    if gpp.deleted {
                        action = "deleted".to_owned();
                    }
                    msg = format!("<{}|[{}]> {} {} commit{} pushed to {} by {}", gpp.repository.url, gpp.repository.full_name,  gpp.commits.len(), action, multiple_commits_msg, branch, gpp.sender.login);
                }
            }
            GitRef::Tag(tag) => {
                msg = format!("<{}|[{}]> Tag '{}' {} by {}", gpp.repository.url, gpp.repository.full_name, tag, action, gpp.sender.login);
            }
        }

        Ok(msg)
    }
}

mod slack {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SlackRequestBody {
        pub blocks: Vec<Block>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Block {
        #[serde(rename = "type")]
        pub type_field: String,
        pub text: Option<Text>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Text {
        #[serde(rename = "type")]
        pub type_field: String,
        pub text: String,
    }

}