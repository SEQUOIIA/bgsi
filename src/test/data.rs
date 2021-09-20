pub const RULES_TEST_DATA : &str = r#"
- name: Allow all of sequoiia
  supplier: github-sequoiia
  action: allow
  accounts:
    - sequoiia
  repos:
    - sequoiia/bgsi
- name: Ignore automated repos from sequoiia
  supplier: github-sequoiia
  action: deny
  repos:
    - sequoiia/bgsi
"#;

pub const RECEIVERS_TEST_DATA : &str = r#"
- name: emcla-notifications-slack
  provider:
    type: slack
    data:
      webhook_url: http://localhost:8080/api/test-incoming
"#;

pub const SUPPLIERS_TEST_DATA : &str = r#"
- name: github-sequoiia
  secret: placeholder
  receivers:
    - emcla-notifications-slack
"#;

pub const GITHUB_PUSH_WEBHOOK_PAYLOAD : &str = r#"
{
  "ref": "refs/tags/simple-tag",
  "before": "6113728f27ae82c7b1a177c8d03f9e96e0adf246",
  "after": "0000000000000000000000000000000000000000",
  "created": false,
  "deleted": true,
  "forced": false,
  "base_ref": null,
  "compare": "https://github.com/sequoiia/bgsi/compare/6113728f27ae...000000000000",
  "commits": [],
  "head_commit": null,
  "repository": {
    "id": 186853002,
    "node_id": "MDEwOlJlcG9zaXRvcnkxODY4NTMwMDI=",
    "name": "bgsi",
    "full_name": "sequoiia/bgsi",
    "private": false,
    "owner": {
      "name": "SEQUOIIA",
      "email": "21031067+SEQUOIIA@users.noreply.github.com",
      "login": "SEQUOIIA",
      "id": 21031067,
      "node_id": "MDQ6VXNlcjIxMDMxMDY3",
      "avatar_url": "https://avatars1.githubusercontent.com/u/1633308?v=4",
      "gravatar_id": "",
      "url": "https://api.github.com/users/SEQUOIIA",
      "html_url": "https://github.com/SEQUOIIA",
      "followers_url": "https://api.github.com/users/SEQUOIIA/followers",
      "following_url": "https://api.github.com/users/SEQUOIIA/following{/other_user}",
      "gists_url": "https://api.github.com/users/SEQUOIIA/gists{/gist_id}",
      "starred_url": "https://api.github.com/users/SEQUOIIA/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/SEQUOIIA/subscriptions",
      "organizations_url": "https://api.github.com/users/SEQUOIIA/orgs",
      "repos_url": "https://api.github.com/users/SEQUOIIA/repos",
      "events_url": "https://api.github.com/users/SEQUOIIA/events{/privacy}",
      "received_events_url": "https://api.github.com/users/SEQUOIIA/received_events",
      "type": "User",
      "site_admin": false
    },
    "html_url": "https://github.com/SEQUOIIA/bgsi",
    "description": null,
    "fork": false,
    "url": "https://github.com/SEQUOIIA/bgsi",
    "forks_url": "https://api.github.com/repos/SEQUOIIA/bgsi/forks",
    "keys_url": "https://api.github.com/repos/SEQUOIIA/bgsi/keys{/key_id}",
    "collaborators_url": "https://api.github.com/repos/SEQUOIIA/bgsi/collaborators{/collaborator}",
    "teams_url": "https://api.github.com/repos/SEQUOIIA/bgsi/teams",
    "hooks_url": "https://api.github.com/repos/SEQUOIIA/bgsi/hooks",
    "issue_events_url": "https://api.github.com/repos/SEQUOIIA/bgsi/issues/events{/number}",
    "events_url": "https://api.github.com/repos/SEQUOIIA/bgsi/events",
    "assignees_url": "https://api.github.com/repos/SEQUOIIA/bgsi/assignees{/user}",
    "branches_url": "https://api.github.com/repos/SEQUOIIA/bgsi/branches{/branch}",
    "tags_url": "https://api.github.com/repos/SEQUOIIA/bgsi/tags",
    "blobs_url": "https://api.github.com/repos/SEQUOIIA/bgsi/git/blobs{/sha}",
    "git_tags_url": "https://api.github.com/repos/SEQUOIIA/bgsi/git/tags{/sha}",
    "git_refs_url": "https://api.github.com/repos/SEQUOIIA/bgsi/git/refs{/sha}",
    "trees_url": "https://api.github.com/repos/SEQUOIIA/bgsi/git/trees{/sha}",
    "statuses_url": "https://api.github.com/repos/SEQUOIIA/bgsi/statuses/{sha}",
    "languages_url": "https://api.github.com/repos/SEQUOIIA/bgsi/languages",
    "stargazers_url": "https://api.github.com/repos/SEQUOIIA/bgsi/stargazers",
    "contributors_url": "https://api.github.com/repos/SEQUOIIA/bgsi/contributors",
    "subscribers_url": "https://api.github.com/repos/SEQUOIIA/bgsi/subscribers",
    "subscription_url": "https://api.github.com/repos/SEQUOIIA/bgsi/subscription",
    "commits_url": "https://api.github.com/repos/SEQUOIIA/bgsi/commits{/sha}",
    "git_commits_url": "https://api.github.com/repos/SEQUOIIA/bgsi/git/commits{/sha}",
    "comments_url": "https://api.github.com/repos/SEQUOIIA/bgsi/comments{/number}",
    "issue_comment_url": "https://api.github.com/repos/SEQUOIIA/bgsi/issues/comments{/number}",
    "contents_url": "https://api.github.com/repos/SEQUOIIA/bgsi/contents/{+path}",
    "compare_url": "https://api.github.com/repos/SEQUOIIA/bgsi/compare/{base}...{head}",
    "merges_url": "https://api.github.com/repos/SEQUOIIA/bgsi/merges",
    "archive_url": "https://api.github.com/repos/SEQUOIIA/bgsi/{archive_format}{/ref}",
    "downloads_url": "https://api.github.com/repos/SEQUOIIA/bgsi/downloads",
    "issues_url": "https://api.github.com/repos/SEQUOIIA/bgsi/issues{/number}",
    "pulls_url": "https://api.github.com/repos/SEQUOIIA/bgsi/pulls{/number}",
    "milestones_url": "https://api.github.com/repos/SEQUOIIA/bgsi/milestones{/number}",
    "notifications_url": "https://api.github.com/repos/SEQUOIIA/bgsi/notifications{?since,all,participating}",
    "labels_url": "https://api.github.com/repos/SEQUOIIA/bgsi/labels{/name}",
    "releases_url": "https://api.github.com/repos/SEQUOIIA/bgsi/releases{/id}",
    "deployments_url": "https://api.github.com/repos/SEQUOIIA/bgsi/deployments",
    "created_at": 1557933565,
    "updated_at": "2019-05-15T15:20:41Z",
    "pushed_at": 1557933657,
    "git_url": "git://github.com/SEQUOIIA/bgsi.git",
    "ssh_url": "git@github.com:SEQUOIIA/bgsi.git",
    "clone_url": "https://github.com/SEQUOIIA/bgsi.git",
    "svn_url": "https://github.com/SEQUOIIA/bgsi",
    "homepage": null,
    "size": 0,
    "stargazers_count": 0,
    "watchers_count": 0,
    "language": "Rust",
    "has_issues": true,
    "has_projects": true,
    "has_downloads": true,
    "has_wiki": true,
    "has_pages": true,
    "forks_count": 1,
    "mirror_url": null,
    "archived": false,
    "disabled": false,
    "open_issues_count": 2,
    "license": null,
    "forks": 1,
    "open_issues": 2,
    "watchers": 0,
    "default_branch": "master",
    "stargazers": 0,
    "master_branch": "master"
  },
  "pusher": {
    "name": "SEQUOIIA",
    "email": "21031067+SEQUOIIA@users.noreply.github.com"
  },
  "sender": {
    "login": "SEQUOIIA",
    "id": 21031067,
    "node_id": "MDQ6VXNlcjIxMDMxMDY3",
    "avatar_url": "https://avatars1.githubusercontent.com/u/21031067?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/SEQUOIIA",
    "html_url": "https://github.com/SEQUOIIA",
    "followers_url": "https://api.github.com/users/SEQUOIIA/followers",
    "following_url": "https://api.github.com/users/SEQUOIIA/following{/other_user}",
    "gists_url": "https://api.github.com/users/SEQUOIIA/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/SEQUOIIA/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/SEQUOIIA/subscriptions",
    "organizations_url": "https://api.github.com/users/SEQUOIIA/orgs",
    "repos_url": "https://api.github.com/users/SEQUOIIA/repos",
    "events_url": "https://api.github.com/users/SEQUOIIA/events{/privacy}",
    "received_events_url": "https://api.github.com/users/SEQUOIIA/received_events",
    "type": "User",
    "site_admin": false
  }
}
"#;