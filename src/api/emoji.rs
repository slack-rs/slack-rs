//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Lists custom emoji for a team.
///
/// Wraps https://api.slack.com/methods/emoji.list
pub fn list(client: &hyper::Client, token: &str) -> ApiResult<ListResponse> {
    make_authed_api_call(client, "emoji.list", token, HashMap::new())
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub emoji: HashMap<String, String>
}

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = list(&client, "TEST_TOKEN");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockListOkResponder,
        r#"{
            "ok": true,
            "emoji": {
                "bowtie": "https:\/\/my.slack.com\/emoji\/bowtie\/46ec6f2bb0.png",
                "squirrel": "https:\/\/my.slack.com\/emoji\/squirrel\/f35f40c0e0.png",
                "shipit": "alias:squirrel"
            }
        }"#
    );

    #[test]
    fn test_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().emoji.get("shipit").unwrap(), "alias:squirrel");
    }
}
