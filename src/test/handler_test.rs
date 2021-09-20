use crate::handler::handle;
use crate::config::Data;
use std::sync::Arc;
use crate::test::data::*;

#[test]
fn test_handle() {
    let data = Arc::new(Data::new_from_bytes(RULES_TEST_DATA.as_bytes(), RECEIVERS_TEST_DATA.as_bytes(), SUPPLIERS_TEST_DATA.as_bytes()));
    handle("placeholder".to_owned(), data, GITHUB_PUSH_WEBHOOK_PAYLOAD.to_string().into_bytes()).unwrap();
}

