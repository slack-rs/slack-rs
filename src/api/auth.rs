//! Checks authentication & identity.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Checks authentication & identity.
///
/// Wraps https://api.slack.com/methods/auth.test
pub fn test(client: &hyper::Client, token: &str) -> ApiResult<TestResponse> {
    make_authed_api_call(client, "auth.test", token, HashMap::new())
}

#[derive(RustcDecodable)]
pub struct TestResponse {
    pub url: String,
    pub team: String,
    pub user: String,
    pub team_id: String,
    pub user_id: String,
}

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = test(&client, "TEST_TOKEN");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockTestOkResponder,
        r#"{
            "ok": true,
            "url": "https:\/\/example-team.slack.com\/",
            "team": "example team",
            "user": "testuser",
            "team_id": "T12345678",
            "user_id": "U12345678"
        }"#
    );

    #[test]
    fn test_ok_response() {
        let client = hyper::Client::with_connector(MockTestOkResponder::default());
        let result = test(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().user, "testuser");
    }
}
