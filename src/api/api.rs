//! Checks API calling code.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test
pub fn test(client: &hyper::Client, token: &str, args: Option<HashMap<&str, &str>>, error: Option<&str>) -> ApiResult<ApiTestResponse> {
    let mut params = HashMap::new();
    if let Some(error) = error {
        params.insert("error", error);
    }
    if let Some(args) = args {
        params.extend(args);
    }
    make_authed_api_call(client, "api.test", token, params)
}

#[derive(RustcDecodable)]
pub struct ApiTestResponse {
    pub error: Option<String>,
    pub args: Option<HashMap<String, String>>
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = test(&client, "TEST_TOKEN", None, Some("some_error"));
        assert!(result.is_err());
    }

    mock_slack_responder!(MockTestOkResponder,
        r#"{
            "ok": true,
            "args": {
                "arg1": "val1",
                "arg2": "val2"
            }
        }"#
    );

    #[test]
    fn test_ok_response() {
        let client = hyper::Client::with_connector(MockTestOkResponder::default());
        let mut args = HashMap::new();
        args.insert("arg1", "val1");
        args.insert("arg2", "val2");
        let result = test(&client, "TEST_TOKEN", Some(args), None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().args.unwrap().get("arg1").unwrap(), "val1");
    }
}
