use crate::handler::handle;
use crate::config::Data;
use std::sync::Arc;
use crate::test::data::*;

#[test]
fn test_handle_1_new_commit() {
    let data = Arc::new(Data::new_from_bytes(RULES_TEST_DATA.as_bytes(), RECEIVERS_TEST_DATA.as_bytes(), SUPPLIERS_TEST_DATA.as_bytes()));
    let resp = handle("placeholder".to_owned(), data, GITHUB_PUSH_WEBHOOK_PAYLOAD_1_NEW_COMMIT.to_string().into_bytes()).unwrap();

    assert_eq!(resp.msg, "[sequoiia/bgsi] 1 new commit pushed to upstream by SEQUOIIA")
}

#[test]
fn test_handle_2_new_commits() {
    let data = Arc::new(Data::new_from_bytes(RULES_TEST_DATA.as_bytes(), RECEIVERS_TEST_DATA.as_bytes(), SUPPLIERS_TEST_DATA.as_bytes()));
    let resp = handle("placeholder".to_owned(), data, GITHUB_PUSH_WEBHOOK_PAYLOAD_2_NEW_COMMITS.to_string().into_bytes()).unwrap();

    assert_eq!(resp.msg, "[sequoiia/bgsi] 2 new commits pushed to upstream by SEQUOIIA")
}

#[test]
fn test_handle_1_new_tag() {
    let data = Arc::new(Data::new_from_bytes(RULES_TEST_DATA.as_bytes(), RECEIVERS_TEST_DATA.as_bytes(), SUPPLIERS_TEST_DATA.as_bytes()));
    let resp = handle("placeholder".to_owned(), data, GITHUB_PUSH_WEBHOOK_PAYLOAD.to_string().into_bytes()).unwrap();

    assert_eq!(resp.msg, "[sequoiia/bgsi] Tag 'simple-tag' removed by SEQUOIIA")
}
