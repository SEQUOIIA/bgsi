use serde::{Deserialize, Serialize};
use crate::model::STResult;
use crate::model::github_push::GitRef::{Branch, Tag};

pub enum GitRef {
    Branch(String),
    Tag(String)
}

impl GitRef {
    pub fn from_string(val : String) -> STResult<Self> {
        Self::from_str(&val)
    }

    pub fn from_str(val : &str) -> STResult<Self> {
        let split : Vec<&str> = val.split('/').collect();
        if split.len() > 2 {
            if split[1].eq("heads") {
                return Ok(Branch(split[2].to_owned()));
            }
            if split[1].eq("tags") {
                return Ok(Tag(split[2].to_owned()));
            }
        }

        Err("GitRef was not a branch nor tag.".into())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubPushPayload {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: ::serde_json::Value,
    pub compare: String,
    pub commits: Vec<::serde_json::Value>,
    pub head_commit: ::serde_json::Value,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: Sender,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: Owner,
    pub html_url: String,
    pub description: ::serde_json::Value,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: i64,
    pub updated_at: String,
    pub pushed_at: i64,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: ::serde_json::Value,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: String,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: ::serde_json::Value,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i64,
    pub license: ::serde_json::Value,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
    pub stargazers: i64,
    pub master_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub name: String,
    pub email: Option<String>,
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sender {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}
