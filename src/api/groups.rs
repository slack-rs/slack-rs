//! Get info on your team's private groups.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Archives a private group.
///
/// Wraps https://api.slack.com/methods/groups.archive
pub fn archive(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<ArchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.archive", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ArchiveResponse;

/// Closes a private group.
///
/// Wraps https://api.slack.com/methods/groups.close
pub fn close(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<CloseResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.close", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CloseResponse {
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>
}

/// Creates a private group.
///
/// Wraps https://api.slack.com/methods/groups.create
pub fn create(client: &hyper::Client, token: &str, name: &str) -> ApiResult<CreateResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    make_authed_api_call(client, "groups.create", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CreateResponse {
    pub group: super::Group
}

/// Clones and archives a private group.
///
/// Wraps https://api.slack.com/methods/groups.createChild
pub fn create_child(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<CreateChildResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.createChild", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CreateChildResponse {
    pub group: super::Group
}

/// Fetches history of messages and events from a private group.
///
/// Wraps https://api.slack.com/methods/groups.history
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
    make_authed_api_call(client, "groups.history", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct HistoryResponse {
    pub latest: Option<String>,
    pub oldest: Option<String>,
    pub messages: Vec<super::Message>,
    pub has_more: bool,
    pub is_limited: Option<bool>
}

/// Gets information about a private group.
///
/// Wraps https://api.slack.com/methods/groups.info
pub fn info(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.info", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub group: super::Group
}

/// Invites a user to a private group.
///
/// Wraps https://api.slack.com/methods/groups.invite
pub fn invite(client: &hyper::Client, token: &str, channel: &str, user: &str) -> ApiResult<InviteResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    make_authed_api_call(client, "groups.invite", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InviteResponse {
    pub group: super::Group,
    pub already_in_group: Option<bool>
}

/// Removes a user from a private group.
///
/// Wraps https://api.slack.com/methods/groups.kick
pub fn kick(client: &hyper::Client, token: &str, channel: &str, user: &str) -> ApiResult<KickResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    make_authed_api_call(client, "groups.kick", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct KickResponse;

/// Leaves a private group.
///
/// Wraps https://api.slack.com/methods/groups.leave
pub fn leave(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<LeaveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.leave", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct LeaveResponse;

/// Lists private groups that the calling user has access to.
///
/// Wraps https://api.slack.com/methods/groups.list
pub fn list(client: &hyper::Client, token: &str, exclude_archived: Option<bool>) -> ApiResult<ListResponse> {
    let mut params = HashMap::new();
    if let Some(exclude_archived) = exclude_archived {
        params.insert("exclude_archived", if exclude_archived { "1" } else { "0" });
    }
    make_authed_api_call(client, "groups.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub groups: Vec<super::Group>
}

/// Sets the read cursor in a private group.
///
/// Wraps https://api.slack.com/methods/groups.mark
pub fn mark(client: &hyper::Client, token: &str, channel: &str, ts: &str) -> ApiResult<MarkResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("ts", ts);
    make_authed_api_call(client, "groups.mark", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MarkResponse;

/// Opens a private group.
///
/// Wraps https://api.slack.com/methods/groups.open
pub fn open(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<OpenResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.open", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct OpenResponse {
    pub no_op: Option<bool>,
    pub already_open: Option<bool>
}

/// Renames a private group.
///
/// Wraps https://api.slack.com/methods/groups.rename
pub fn rename(client: &hyper::Client, token: &str, channel: &str, name: &str) -> ApiResult<RenameResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("name", name);
    make_authed_api_call(client, "groups.rename", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AbridgedGroup {
    pub id: String,
    pub is_group: bool,
    pub name: String,
    pub created: i32,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RenameResponse {
    pub channel: AbridgedGroup
}

/// Sets the purpose for a private group.
///
/// Wraps https://api.slack.com/methods/groups.setPurpose
pub fn set_purpose(client: &hyper::Client, token: &str, channel: &str, purpose: &str) -> ApiResult<SetPurposeResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("purpose", purpose);
    make_authed_api_call(client, "groups.setPurpose", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetPurposeResponse {
    pub purpose: String
}

/// Sets the topic for a private group.
///
/// Wraps https://api.slack.com/methods/groups.setTopic
pub fn set_topic(client: &hyper::Client, token: &str, channel: &str, topic: &str) -> ApiResult<SetTopicResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("topic", topic);
    make_authed_api_call(client, "groups.setTopic", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetTopicResponse {
    pub topic: String
}

/// Unarchives a private group.
///
/// Wraps https://api.slack.com/methods/groups.unarchive
pub fn unarchive(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<UnarchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "groups.unarchive", token, params)
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
        let result = info(&client, "TEST_TOKEN", "G12345678");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockArchiveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn archive_ok_response() {
        let client = hyper::Client::with_connector(MockArchiveOkResponder::default());
        let result = archive(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockCloseOkResponder, r#"{"ok": true}"#);

    #[test]
    fn close_ok_response() {
        let client = hyper::Client::with_connector(MockCloseOkResponder::default());
        let result = close(&client, "TEST_TOKEN", "G12345678");
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
        let result = close(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().already_closed.unwrap(), true);
    }

    mock_slack_responder!(MockCreateOkResponder,
        r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#
    );

    #[test]
    fn create_ok_response() {
        let client = hyper::Client::with_connector(MockCreateOkResponder::default());
        let result = create(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }

    mock_slack_responder!(MockCreateChildOkResponder,
        r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#
    );

    #[test]
    fn create_child_ok_response() {
        let client = hyper::Client::with_connector(MockCreateChildOkResponder::default());
        let result = create_child(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
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
        let result = history(&client, "TEST_TOKEN", "G12345678", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().messages[0].clone() {
            MessageEvent::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "Hello")
            },
            _ => panic!("Message decoded into incorrect variant.")
        }
    }

    mock_slack_responder!(MockInfoOkResponder,
        r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#
    );

    #[test]
    fn info_ok_response() {
        let client = hyper::Client::with_connector(MockInfoOkResponder::default());
        let result = info(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }

    mock_slack_responder!(MockInviteOkResponder,
        r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#
    );

    #[test]
    fn invite_ok_response() {
        let client = hyper::Client::with_connector(MockInviteOkResponder::default());
        let result = invite(&client, "TEST_TOKEN", "G12345678", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }

    mock_slack_responder!(MockInviteAlreadyInGroupOkResponder,
        r#"{
            "ok": true,
            "already_in_group": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#
    );

    #[test]
    fn invite_already_in_channel_ok_response() {
        let client = hyper::Client::with_connector(MockInviteAlreadyInGroupOkResponder::default());
        let result = invite(&client, "TEST_TOKEN", "G12345678", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().already_in_group.unwrap() == true);
    }

    mock_slack_responder!(MockKickOkResponder, r#"{"ok": true}"#);

    #[test]
    fn kick_ok_response() {
        let client = hyper::Client::with_connector(MockKickOkResponder::default());
        let result = kick(&client, "TEST_TOKEN", "G12345678", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockLeaveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn leave_ok_response() {
        let client = hyper::Client::with_connector(MockLeaveOkResponder::default());
        let result = leave(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockListOkResponder,
        r#"{
            "ok": true,
            "groups": [
                {
                    "id": "G12345678",
                    "name": "secretplans",
                    "is_group": true,
                    "created": 1360782804,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_open": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "Secret plans on hold",
                        "creator": "U024BE7LV",
                        "last_set": 1369677212
                    },
                    "purpose": {
                        "value": "Discuss secret plans that no-one else should know",
                        "creator": "U024BE7LH",
                        "last_set": 1360782804
                    }
                },
                {
                    "id": "G87654321",
                    "name": "secretplans",
                    "is_group": true,
                    "created": 1360782804,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_open": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "Secret plans on hold",
                        "creator": "U024BE7LV",
                        "last_set": 1369677212
                    },
                    "purpose": {
                        "value": "Discuss secret plans that no-one else should know",
                        "creator": "U024BE7LH",
                        "last_set": 1360782804
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
        assert!(result.unwrap().groups[1].id == "G87654321");
    }

    mock_slack_responder!(MockMarkOkResponder, r#"{"ok": true}"#);

    #[test]
    fn mark_ok_response() {
        let client = hyper::Client::with_connector(MockMarkOkResponder::default());
        let result = mark(&client, "TEST_TOKEN", "G12345678", "1234567890.123456");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockRenameOkResponder,
        r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91J",
                "is_group": true,
                "name": "NEW_NAME",
                "created": 1444102158
            }
        }"#
    );

    #[test]
    fn rename_ok_response() {
        let client = hyper::Client::with_connector(MockRenameOkResponder::default());
        let result = rename(&client, "TEST_TOKEN", "G12345678", "newname");
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
        let result = set_purpose(&client, "TEST_TOKEN", "G12345678", "This is the new purpose!");
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
        let result = set_topic(&client, "TEST_TOKEN", "G12345678", "This is the new topic!");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().topic == "This is the new topic!")
    }

    mock_slack_responder!(MockUnarchiveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn unarchive_ok_response() {
        let client = hyper::Client::with_connector(MockUnarchiveOkResponder::default());
        let result = unarchive(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
