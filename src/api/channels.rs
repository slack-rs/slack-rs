//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic
//! and purpose, and mark a channel as read.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Archives a channel.
///
/// Wraps https://api.slack.com/methods/channels.archive
pub fn archive(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<ArchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "channels.archive", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ArchiveResponse;

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create
pub fn create(client: &hyper::Client, token: &str, name: &str) -> ApiResult<CreateResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    make_authed_api_call(client, "channels.create", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CreateResponse {
    pub channel: super::Channel,
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history
pub fn history(client: &hyper::Client, token: &str, channel: &str, latest: Option<&str>, oldest: Option<&str>, inclusive: Option<bool>, count: Option<u32>) -> ApiResult<HistoryResponse> {
    let count = count.map(|c| c.to_string());
    let mut params = HashMap::new();
    params.insert("channel", channel);
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
    make_authed_api_call(client, "channels.history", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct HistoryResponse {
    pub latest: Option<String>,
    pub oldest: Option<String>,
    pub messages: Vec<super::Message>,
    pub has_more: bool,
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info
pub fn info(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "channels.info", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    channel: super::Channel,
}

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite
pub fn invite(client: &hyper::Client, token: &str, channel: &str, user: &str) -> ApiResult<InviteResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    make_authed_api_call(client, "channels.invite", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InviteResponse {
    channel: super::Channel,
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join
pub fn join(client: &hyper::Client, token: &str, name: &str) -> ApiResult<JoinResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    make_authed_api_call(client, "channels.join", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct JoinResponse {
    already_in_channel: Option<bool>,
    channel: super::Channel,
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick
pub fn kick(client: &hyper::Client, token: &str, channel: &str, user: &str) -> ApiResult<KickResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    make_authed_api_call(client, "channels.kick", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct KickResponse;

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave
pub fn leave(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<LeaveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "channels.leave", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct LeaveResponse {
    pub not_in_channel: Option<bool>,
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list
pub fn list(client: &hyper::Client, token: &str, exclude_archived: Option<bool>) -> ApiResult<ListResponse> {
    let mut params = HashMap::new();
    if let Some(exclude_archived) = exclude_archived {
        params.insert("exclude_archived", if exclude_archived { "1" } else { "0" });
    }
    make_authed_api_call(client, "channels.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub channels: Vec<super::Channel>,
}

/// Sets the read cursor in a channel.
///
/// https://api.slack.com/methods/channels.mark
pub fn mark(client: &hyper::Client, token: &str, channel: &str, ts: &str) -> ApiResult<MarkResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("ts", ts);
    make_authed_api_call(client, "channels.mark", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MarkResponse;

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename
pub fn rename(client: &hyper::Client, token: &str, channel: &str, name: &str) -> ApiResult<RenameResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("name", name);
    make_authed_api_call(client, "channels.rename", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AbridgedChannel {
    pub id: String,
    pub is_channel: bool,
    pub name: String,
    pub created: i32,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RenameResponse {
    pub channel: AbridgedChannel,
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose
pub fn set_purpose(client: &hyper::Client, token: &str, channel: &str, purpose: &str) -> ApiResult<SetPurposeResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("purpose", purpose);
    make_authed_api_call(client, "channels.setPurpose", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetPurposeResponse {
    pub purpose: String,
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic
pub fn set_topic(client: &hyper::Client, token: &str, channel: &str, topic: &str) -> ApiResult<SetTopicResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("topic", topic);
    make_authed_api_call(client, "channels.setTopic", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetTopicResponse {
    pub topic: String,
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive
pub fn unarchive(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<UnarchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "channels.unarchive", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct UnarchiveResponse;

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;
    use super::super::MessageEvent;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = info(&client, "TEST_TOKEN", "TEST_CHANNEL");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockArchiveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn archive_ok_response() {
        let client = hyper::Client::with_connector(MockArchiveOkResponder::default());
        let result = archive(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockCreateOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#
    );

    #[test]
    fn create_ok_response() {
        let client = hyper::Client::with_connector(MockCreateOkResponder::default());
        let result = create(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    mock_slack_responder!(MockHistoryOkResponder,
        r#"{
            "ok": true,
            "messages": [
                {
                    "type": "message",
                    "user": "U024BE7LH",
                    "text": "lol",
                    "ts": "1444078138.000084"
                },
                {
                    "type": "message",
                    "user": "U024BE7LH",
                    "text": "Hello world",
                    "ts": "1444078079.000083"
                }
            ],
            "has_more": true
        }"#
    );

    #[test]
    fn history_ok_response() {
        let client = hyper::Client::with_connector(MockHistoryOkResponder::default());
        let result = history(&client, "TEST_TOKEN", "TEST_CHANNEL", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().messages[0].clone() {
            MessageEvent::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "lol");
            },
            _ => panic!("Message decoded into incorrect variant.")
        }
    }

    mock_slack_responder!(MockInfoOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#
    );

    #[test]
    fn info_ok_response() {
        let client = hyper::Client::with_connector(MockInfoOkResponder::default());
        let result = info(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    mock_slack_responder!(MockInviteOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#
    );

    #[test]
    fn invite_ok_response() {
        let client = hyper::Client::with_connector(MockInviteOkResponder::default());
        let result = invite(&client, "TEST_TOKEN", "TEST_CHANNEL", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    mock_slack_responder!(MockJoinOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#
    );

    #[test]
    fn join_ok_response() {
        let client = hyper::Client::with_connector(MockJoinOkResponder::default());
        let result = join(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    mock_slack_responder!(MockKickOkResponder, r#"{"ok": true}"#);

    #[test]
    fn kick_ok_response() {
        let client = hyper::Client::with_connector(MockKickOkResponder::default());
        let result = kick(&client, "TEST_TOKEN", "TEST_CHANNEL", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockLeaveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn leave_ok_response() {
        let client = hyper::Client::with_connector(MockLeaveOkResponder::default());
        let result = leave(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockListOkResponder,
        r#"{
            "ok": true,
            "channels": [
                {
                    "id": "C024BE91L",
                    "name": "testing",
                    "is_channel": true,
                    "created": 1444102158,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_general": false,
                    "is_member": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    },
                    "purpose": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    }
                },
                {
                    "id": "C024BE91J",
                    "name": "testing",
                    "is_channel": true,
                    "created": 1444102158,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_general": false,
                    "is_member": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    },
                    "purpose": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    }
                }
            ]
        }"#
    );

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN", None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channels[1].id == "C024BE91J");
    }

    mock_slack_responder!(MockMarkOkResponder, r#"{"ok": true}"#);

    #[test]
    fn mark_ok_response() {
        let client = hyper::Client::with_connector(MockMarkOkResponder::default());
        let result = mark(&client, "TEST_TOKEN", "TEST_CHANNEL", "1234567890.123456");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockRenameOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91J",
                "is_channel": true,
                "name": "NEW_NAME",
                "created": 1444102158
            }
        }"#
    );

    #[test]
    fn rename_ok_response() {
        let client = hyper::Client::with_connector(MockRenameOkResponder::default());
        let result = rename(&client, "TEST_TOKEN", "TEST_CHANNEL", "newname");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.name == "NEW_NAME")
    }

    mock_slack_responder!(MockSetPurposeOkResponder,
        r#"{
            "ok": true,
            "purpose": "This is the new purpose!"
        }"#
    );

    #[test]
    fn set_purpose_ok_response() {
        let client = hyper::Client::with_connector(MockSetPurposeOkResponder::default());
        let result = set_purpose(&client, "TEST_TOKEN", "TEST_CHANNEL", "This is the new purpose!");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().purpose == "This is the new purpose!")
    }

    mock_slack_responder!(MockSetTopicOkResponder,
        r#"{
            "ok": true,
            "topic": "This is the new topic!"
        }"#
    );

    #[test]
    fn set_topic_ok_response() {
        let client = hyper::Client::with_connector(MockSetTopicOkResponder::default());
        let result = set_topic(&client,
                               "TEST_TOKEN",
                               "TEST_CHANNEL",
                               "This is the new topic!");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().topic == "This is the new topic!")
    }

    mock_slack_responder!(MockUnarchiveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn unarchive_ok_response() {
        let client = hyper::Client::with_connector(MockUnarchiveOkResponder::default());
        let result = unarchive(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
