//! Post chat messages to Slack.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete
pub fn delete(client: &hyper::Client, token: &str, ts: &str, channel: &str) -> ApiResult<DeleteResponse> {
    let mut params = HashMap::new();
    params.insert("ts", ts);
    params.insert("channel", channel);
    make_authed_api_call(client, "chat.delete", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct DeleteResponse {
    pub channel: String,
    pub ts: String,
}

/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage
pub fn post_message(client: &hyper::Client, token: &str, channel: &str, text: &str, username: Option<&str>, as_user: Option<bool>, parse: Option<&str>, link_names: Option<bool>, attachments: Option<&str>, unfurl_links: Option<bool>, unfurl_media: Option<bool>, icon_url: Option<&str>, icon_emoji: Option<&str>) -> ApiResult<PostMessageResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("text", text);
    if let Some(username) = username {
        params.insert("username", username);
    }
    if let Some(as_user) = as_user {
        params.insert("as_user", if as_user { "true" } else { "false" });
    }
    if let Some(parse) = parse {
        params.insert("parse", parse);
    }
    if let Some(link_names) = link_names {
        params.insert("link_names", if link_names { "1" } else { "0" });
    }
    if let Some(attachments) = attachments {
        params.insert("attachments", attachments);
    }
    if let Some(unfurl_links) = unfurl_links {
        params.insert("unfurl_links", if unfurl_links { "true" } else { "false" });
    }
    if let Some(unfurl_media) = unfurl_media {
        params.insert("unfurl_media", if unfurl_media { "true" } else { "false" });
    }
    if let Some(icon_url) = icon_url {
        params.insert("icon_url", icon_url);
    }
    if let Some(icon_emoji) = icon_emoji {
        params.insert("icon_emoji", icon_emoji);
    }
    make_authed_api_call(client, "chat.postMessage", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct PostMessageResponse {
    pub ts: String,
    pub channel: String,
    pub message: super::Message,
}

/// Updates a message.
///
/// Wraps https://api.slack.com/methods/chat.update
pub fn update(client: &hyper::Client, token: &str, ts: &str, channel: &str, text: &str, attachments: Option<&str>, parse: Option<&str>, link_names: Option<bool>) -> ApiResult<UpdateResponse> {
    let mut params = HashMap::new();
    params.insert("ts", ts);
    params.insert("channel", channel);
    params.insert("text", text);
    if let Some(attachments) = attachments {
        params.insert("attachments", attachments);
    }
    if let Some(parse) = parse {
        params.insert("parse", parse);
    }
    if let Some(link_names) = link_names {
        params.insert("link_names", if link_names { "1" } else { "0" });
    }
    make_authed_api_call(client, "chat.update", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct UpdateResponse {
    pub channel: String,
    pub ts: String,
    pub text: String,
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
        let result = post_message(&client, "TEST_TOKEN", "TEST_CHANNEL", "Test message", None, None, None, None, None, None, None, None, None);
        assert!(result.is_err());
    }

    mock_slack_responder!(MockDeleteResponder, r#"{
        "ok": true,
        "channel": "C024BE91L",
        "ts": "1401383885.000061"
    }"#);

    #[test]
    fn delete_ok_response() {
        let client = hyper::Client::with_connector(MockDeleteResponder::default());
        let result = delete(&client, "TEST_TOKEN", "TEST_CHANNEL", "1401383885.000061");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().ts, "1401383885.000061");
    }

    mock_slack_responder!(MockPostMessageResponder, r#"{
        "ok": true,
        "ts": "1405895017.000506",
        "channel": "C024BE91L",
        "message": {
            "type": "message",
            "user": "U024BE7LH",
            "text": "Test message",
            "ts": "1444078138.000084"
        }
    }"#);

    #[test]
    fn post_message_ok_response() {
        let client = hyper::Client::with_connector(MockPostMessageResponder::default());
        let result = post_message(&client, "TEST_TOKEN", "TEST_CHANNEL", "Test message", None, Some(true), None, None, None, None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().message {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "Test message");
            },
            _ => panic!("Message decoded into incorrect variant.")
        }
    }

    mock_slack_responder!(MockBotPostMessageResponder, r#"{
        "ok": true,
        "ts": "1405895017.000506",
        "channel": "C024BE91L",
        "message": {
            "type": "message",
            "text": "Test message",
            "ts": "1444078138.000084"
        }
    }"#);

    #[test]
    fn bot_post_message_ok_response() {
        let client = hyper::Client::with_connector(MockPostMessageResponder::default());
        let result = post_message(&client, "TEST_TOKEN", "TEST_CHANNEL", "Test message", None, None, None, None, None, None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().message.clone() {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "Test message")
            },
            _ => panic!("Message decoded into incorrect variant.")
        }
    }

    mock_slack_responder!(MockUpdateResponder, r#"{
        "ok": true,
        "channel": "C024BE91L",
        "ts": "1401383885.000061",
        "text": "Test message"
    }"#);

    #[test]
    fn update_ok_response() {
        let client = hyper::Client::with_connector(MockUpdateResponder::default());
        let result = update(&client, "TEST_TOKEN", "TEST_CHANNEL", "1401383885.000061", "Test message", None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().text, "Test message");
    }
}
