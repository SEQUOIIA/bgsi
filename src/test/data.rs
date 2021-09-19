pub const RULES_TEST_DATA : &str = r#"
- name: Ignore automated repos
  supplier: github-sequoiia
  action: allow
  accounts:
    - sequoiia
  repos:
    - bgsi
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