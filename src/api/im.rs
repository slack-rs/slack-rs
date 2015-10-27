//! Get info on your direct messages.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Close a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.close
pub fn close(client: &hyper::Client, token: &str, channel_id: &str) -> ApiResult<CloseResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel_id);
    make_authed_api_call(client, "im.close", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CloseResponse {
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>,
}

/// Fetches history of messages and events from direct message channel.
///
/// Wraps https://api.slack.com/methods/im.history
pub fn history(client: &hyper::Client, token: &str, channel_id: &str, latest: Option<&str>, oldest: Option<&str>, inclusive: Option<bool>, count: Option<u32>) -> ApiResult<HistoryResponse> {
    let count = count.map(|c| c.to_string());
    let mut params = HashMap::new();
    params.insert("channel", channel_id);
    if let Some(latest) = latest {
        params.insert("latest", latest);
    }
    if let Some(oldest) = oldest {
        params.insert("oldest", oldest);
    }
    if let Some(inclusive) = inclusive {
        params.insert("inclusive", if inclusive { "1" } else { "0" });
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    make_authed_api_call(client, "im.history", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct HistoryResponse {
    pub latest: String,
    pub messages: Vec<super::Message>,
    pub has_more: bool,
}

/// Lists direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/im.list
pub fn list(client: &hyper::Client, token: &str) -> ApiResult<ListResponse> {
    make_authed_api_call(client, "im.list", token, HashMap::new())
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub ims: Vec<super::Im>,
}

/// Sets the read cursor in a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.mark
pub fn mark(client: &hyper::Client, token: &str, channel_id: &str, timestamp: &str) -> ApiResult<MarkResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel_id);
    params.insert("timestamp", timestamp);
    make_authed_api_call(client, "im.mark", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MarkResponse;

/// Opens a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.open
pub fn open(client: &hyper::Client, token: &str, user_id: &str) -> ApiResult<OpenResponse> {
    let mut params = HashMap::new();
    params.insert("user", user_id);
    make_authed_api_call(client, "im.open", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ChannelId {
    pub id: String
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct OpenResponse {
    pub no_op: Option<bool>,
    pub already_open: Option<bool>,
    pub channel: ChannelId,
}

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;
    use super::super::Message;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = close(&client, "TEST_TOKEN", "D12345678");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockCloseOkResponder, r#"{"ok": true}"#);

    #[test]
    fn close_ok_response() {
        let client = hyper::Client::with_connector(MockCloseOkResponder::default());
        let result = close(&client, "TEST_TOKEN", "D12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockCloseAlreadyClosedOkResponder,
        r#"{
            "ok": true,
            "no_op": true,
            "already_closed": true
        }"#
    );

    #[test]
    fn close_already_closed_ok_response() {
        let client = hyper::Client::with_connector(MockCloseAlreadyClosedOkResponder::default());
        let result = close(&client, "TEST_TOKEN", "D12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().already_closed.unwrap(), true);
    }

    mock_slack_responder!(MockHistoryOkResponder,
        r#"{
            "ok": true,
            "latest": "1358547726.000003",
            "messages": [
                {
                    "type": "message",
                    "ts": "1358546515.000008",
                    "user": "U2147483896",
                    "text": "Hello"
                },
                {
                    "type": "message",
                    "ts": "1358546515.000007",
                    "user": "U2147483896",
                    "text": "World",
                    "is_starred": true
                },
                {
                    "type": "something_else",
                    "ts": "1358546515.000007",
                    "wibblr": true
                }
            ],
            "has_more": false
        }"#
    );

    #[test]
    fn history_ok_response() {
        let client = hyper::Client::with_connector(MockHistoryOkResponder::default());
        let result = history(&client, "TEST_TOKEN", "D12345678", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().messages[0].clone() {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "Hello");
            },
            _ => panic!("Message decoded into incorrect variant.")
        }
    }

    mock_slack_responder!(MockListOkResponder,
        r#"{
            "ok": true,
            "ims": [
                {
                   "id": "D024BFF1M",
                   "is_im": true,
                   "user": "USLACKBOT",
                   "created": 1372105335,
                   "is_user_deleted": false
                },
                {
                   "id": "D024BE7RE",
                   "is_im": true,
                   "user": "U024BE7LH",
                   "created": 1356250715,
                   "is_user_deleted": false
                }
            ]
        }"#
    );

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().ims[1].id == "D024BE7RE");
    }

    mock_slack_responder!(MockMarkOkResponder, r#"{"ok": true}"#);

    #[test]
    fn mark_ok_response() {
        let client = hyper::Client::with_connector(MockMarkOkResponder::default());
        let result = mark(&client, "TEST_TOKEN", "D12345678", "1234567890.123456");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockOpenOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "D024BFF1M"
            }
        }"#
    );

    #[test]
    fn open_ok_response() {
        let client = hyper::Client::with_connector(MockOpenOkResponder::default());
        let result = open(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().channel.id, "D024BFF1M");
    }

    mock_slack_responder!(MockOpenAlreadyOpenOkResponder,
        r#"{
            "ok": true,
            "no_op": true,
            "already_open": true,
            "channel": {
                "id": "D024BFF1M"
            }
        }"#
    );

    #[test]
    fn open_already_open_ok_response() {
        let client = hyper::Client::with_connector(MockOpenAlreadyOpenOkResponder::default());
        let result = open(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.channel.id, "D024BFF1M");
        assert_eq!(result.already_open.unwrap(), true);
    }
}
