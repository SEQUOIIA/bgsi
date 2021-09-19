use crate::handler::handle;
use crate::config::Data;
use std::sync::Arc;

#[test]
fn test_handle() {
    let data = Arc::new(Data::new_from_bytes(RULES_TEST_DATA.as_bytes(), RECEIVERS_TEST_DATA.as_bytes(), SUPPLIERS_TEST_DATA.as_bytes()));
    handle("placeholder".to_owned(), data).unwrap();
}

const RULES_TEST_DATA : &str = r#"
- name: Ignore automated repos
  supplier: github-sequoiia
  action: allow
  accounts:
    - sequoiia
  repos:
    - bgsi
"#;

const RECEIVERS_TEST_DATA : &str = r#"
- name: emcla-notifications-slack
  provider:
    type: slack
    data:
      webhook_url: http://localhost:8080/api/test-incoming
"#;

const SUPPLIERS_TEST_DATA : &str = r#"
- name: github-sequoiia
  secret: placeholder
  receivers:
    - emcla-notifications-slack
"#;