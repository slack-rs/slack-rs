use rustc_serialize::{Decodable,Decoder};

use std::collections::HashMap;

/// The metadata of an edited [`Message`](https://api.slack.com/events/message).
#[derive(Clone,Debug,RustcDecodable)]
pub struct EditedMessageData {
    pub user: String,
    pub ts: String
}

/// Represents Slack [message event](https://api.slack.com/events/message) types.
#[derive(Clone,Debug)]
pub enum Message {
    /// The Slack [`Message`](https://api.slack.com/events/message) event that represents a message
    /// to a channel, group or im.
    Standard {
        ts: String,
        channel: Option<String>,
        user: Option<String>,
        text: Option<String>,
        is_starred: Option<bool>,
        pinned_to: Option<Vec<String>>,
        reactions: Option<Vec<super::Reaction>>,
        edited: Option<EditedMessageData>,
        attachments: Option<Vec<super::Attachment>>
    },
    /// Wraps the [`bot_message`](https://api.slack.com/events/message/bot_message) message event.
    BotMessage {
        ts: String,
        text: String,
        bot_id: String,
        username: Option<String>,
        icons: Option<HashMap<String,String>>
    },
    /// Wraps the [`me_message`](https://api.slack.com/events/message/me_message) message event.
    MeMessage {
        channel: String,
        user: String,
        text: String,
        ts: String
    },
    /// Wraps the [`message_changed`](https://api.slack.com/events/message/message_changed) message
    /// event.
    MessageChanged {
        hidden: bool,
        channel: String,
        ts: String,
        message: Box<Message>
    },
    /// Wraps the [`message_deleted`](https://api.slack.com/events/message/message_deleted) message
    /// event.
    MessageDeleted {
        hidden: bool,
        channel: String,
        ts: String,
        deleted_ts: String
    },
    /// Wraps the [`channel_join`](https://api.slack.com/events/message/channel_join) message
    /// event.
    ChannelJoin {
        ts: String,
        user: String,
        text: String,
        inviter: Option<String>
    },
    /// Wraps the [`channel_leave`](https://api.slack.com/events/message/channel_leave) message
    /// event.
    ChannelLeave {
        ts: String,
        user: String,
        text: String,
    },
    /// Wraps the [`channel_topic`](https://api.slack.com/events/message/channel_topic) message
    /// event.
    ChannelTopic {
        ts: String,
        user: String,
        topic: String,
        text: String,
    },
    /// Wraps the [`channel_purpose`](https://api.slack.com/events/message/channel_purpose) message
    /// event.
    ChannelPurpose {
        ts: String,
        user: String,
        purpose: String,
        text: String
    },
    /// Wraps the [`channel_name`](https://api.slack.com/events/message/channel_name) message
    /// event.
    ChannelName {
        ts: String,
        user: String,
        old_name: String,
        name: String,
        text: String
    },
    /// Wraps the [`channel_archive`](https://api.slack.com/events/message/channel_archive) message
    /// event.
    ChannelArchive {
        ts: String,
        text: String,
        user: String,
        members: Vec<String>
    },
    /// Wraps the [`channel_unarchive`](https://api.slack.com/events/message/channel_unarchive)
    /// message event.
    ChannelUnarchive {
        ts: String,
        text: String,
        user: String
    },
    /// Wraps the [`group_join`](https://api.slack.com/events/message/group_join) message event.
    GroupJoin {
        ts: String,
        user: String,
        text: String,
        inviter: Option<String>
    },
    /// Wraps the [`group_leave`](https://api.slack.com/events/message/group_leave) message event.
    GroupLeave {
        ts: String,
        user: String,
        text: String
    },
    /// Wraps the [`group_topic`](https://api.slack.com/events/message/group_topic) message event.
    GroupTopic {
        ts: String,
        user: String,
        topic: String,
        text: String
    },
    /// Wraps the [`group_purpose`](https://api.slack.com/events/message/group_purpose) message
    /// event.
    GroupPurpose {
        ts: String,
        user: String,
        purpose: String,
        text: String
    },
    /// Wraps the [`group_name`](https://api.slack.com/events/message/group_name) message event.
    GroupName {
        ts: String,
        user: String,
        old_name: String,
        name: String,
        text: String
    },
    /// Wraps the [`group_archive`](https://api.slack.com/events/message/group_archive) message
    /// event.
    GroupArchive {
        ts: String,
        text: String,
        user: String,
        members: Vec<String>
    },
    /// Wraps the [`group_unarchive`](https://api.slack.com/events/message/group_unarchive)
    /// message event.
    GroupUnarchive {
        ts: String,
        text: String,
        user: String
    },
    /// Wraps the [`file_share`](https://api.slack.com/events/message/file_share) message event.
    FileShare {
        ts: String,
        text: String,
        file: super::File,
        user: String,
        upload: bool
    },
    /// Wraps the [`file_comment`](https://api.slack.com/events/message/file_comment) message
    /// event.
    FileComment {
        ts: String,
        text: String,
        file: super::File,
        comment: super::Comment
    },
    /// Wraps the [`file_mention`](https://api.slack.com/events/message/file_mention) message
    /// event.
    FileMention {
        ts: String,
        text: String,
        file: super::File,
        user: String
    },
    /// Wraps the [`pinned_item`](https://api.slack.com/events/message/pinned_item) message event.
    PinnedItem {
        user: String,
        item_type: String,
        text: String,
        item: Option<super::Item>,
        channel: String,
        ts: String,
        attachments: Option<Vec<super::Attachment>>
    },
    /// Wraps the [`unpinned_item`](https://api.slack.com/events/message/unpinned_item) message
    /// event.
    UnpinnedItem {
        user: String,
        item_type: String,
        text: String,
        item: Option<super::Item>,
        channel: String,
        ts: String,
        attachments: Option<Vec<super::Attachment>>
    }
}

impl Decodable for Message {
    fn decode<D: Decoder>(d: &mut D) -> Result<Message, D::Error> {
        d.read_struct("message", 0, |d| {
            let ty: Option<String> = try!(d.read_struct_field("subtype", 0, |d| Decodable::decode(d)));
            if ty.is_none() {
                return Ok(Message::Standard {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    is_starred: try!(d.read_struct_field("is_starred", 0, |d| Decodable::decode(d))),
                    pinned_to: try!(d.read_struct_field("pinned_to", 0, |d| Decodable::decode(d))),
                    reactions: try!(d.read_struct_field("reactions", 0, |d| Decodable::decode(d))),
                    edited: try!(d.read_struct_field("edited", 0, |d| Decodable::decode(d))),
                    attachments: try!(d.read_struct_field("attachments", 0, |d| Decodable::decode(d)))
                });
            }
            let ty = ty.unwrap();
            if ty == "bot_message" {
                Ok(Message::BotMessage {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    bot_id: try!(d.read_struct_field("bot_id", 0, |d| Decodable::decode(d))),
                    username: try!(d.read_struct_field("username", 0, |d| Decodable::decode(d))),
                    icons: try!(d.read_struct_field("icons", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "me_message" {
                Ok(Message::MeMessage {
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "message_changed" {
                Ok(Message::MessageChanged {
                    hidden: try!(d.read_struct_field("hidden", 0, |d| Decodable::decode(d))),
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    message: try!(d.read_struct_field("message", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "message_deleted" {
                Ok(Message::MessageDeleted {
                    hidden: try!(d.read_struct_field("hidden", 0, |d| Decodable::decode(d))),
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    deleted_ts: try!(d.read_struct_field("deleted_ts", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_join" {
                Ok(Message::ChannelJoin {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    inviter: try!(d.read_struct_field("inviter", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_leave" {
                Ok(Message::ChannelLeave {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_topic" {
                Ok(Message::ChannelTopic {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    topic: try!(d.read_struct_field("topic", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_purpose" {
                Ok(Message::ChannelPurpose {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    purpose: try!(d.read_struct_field("purpose", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_name" {
                Ok(Message::ChannelName {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    old_name: try!(d.read_struct_field("old_name", 0, |d| Decodable::decode(d))),
                    name: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_archive" {
                Ok(Message::ChannelArchive {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    members: try!(d.read_struct_field("members", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel_unarchive" {
                Ok(Message::ChannelUnarchive {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_join" {
                Ok(Message::GroupJoin {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    inviter: try!(d.read_struct_field("inviter", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_leave" {
                Ok(Message::GroupLeave {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_topic" {
                Ok(Message::GroupTopic {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    topic: try!(d.read_struct_field("topic", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_purpose" {
                Ok(Message::GroupPurpose {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    purpose: try!(d.read_struct_field("purpose", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_name" {
                Ok(Message::GroupName {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    old_name: try!(d.read_struct_field("old_name", 0, |d| Decodable::decode(d))),
                    name: try!(d.read_struct_field("name", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_archive" {
                Ok(Message::GroupArchive {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    members: try!(d.read_struct_field("members", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "group_unarchive" {
                Ok(Message::GroupUnarchive {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file_share" {
                Ok(Message::FileShare {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    upload: try!(d.read_struct_field("upload", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file_comment" {
                Ok(Message::FileComment {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    comment: try!(d.read_struct_field("comment", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file_mention" {
                Ok(Message::FileMention {
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "pinned_item" {
                Ok(Message::PinnedItem {
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    item_type: try!(d.read_struct_field("item_type", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    attachments: try!(d.read_struct_field("attachments", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "unpinned_item" {
                Ok(Message::UnpinnedItem {
                    user: try!(d.read_struct_field("user", 0, |d| Decodable::decode(d))),
                    item_type: try!(d.read_struct_field("item_type", 0, |d| Decodable::decode(d))),
                    text: try!(d.read_struct_field("text", 0, |d| Decodable::decode(d))),
                    item: try!(d.read_struct_field("item", 0, |d| Decodable::decode(d))),
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    ts: try!(d.read_struct_field("ts", 0, |d| Decodable::decode(d))),
                    attachments: try!(d.read_struct_field("attachments", 0, |d| Decodable::decode(d)))
                })
            } else {
                Err(d.error(&format!("Unknown Message type: {}", ty)))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Item;
    use rustc_serialize::json;

    #[test]
    fn decode_short_standard_message() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "ts": "1234567890.218332",
            "user": "U12345678",
            "text": "Hello world",
            "channel": "C12345678"
        }"#).unwrap();
        match message {
            Message::Standard { ts, channel: _, user, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(ts, "1234567890.218332");
                assert_eq!(text.unwrap(), "Hello world");
                assert_eq!(user.unwrap(), "U12345678");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_extended_standard_message() {
        let message: Message = json::decode(r##"{
            "type": "message",
            "ts": "1234567890.218332",
            "user": "U12345678",
            "text": "Hello world",
            "channel": "C12345678",
            "is_starred": false,
            "pinned_to": [ "C12345678" ],
            "reactions": [
                {
                    "name": "astonished",
                    "count": 5,
                    "users": [ "U12345678", "U87654321" ]
                }
            ],
            "edited": {
                "user": "U12345678",
                "ts": "1234567890.218332"
            },
            "attachments": [
                {
                    "fallback": "Required plain-text summary of the attachment.",
                    "color": "#36a64f",
                    "pretext": "Optional text that appears above the attachment block",
                    "author_name": "Bobby Tables",
                    "author_link": "http://flickr.com/bobby/",
                    "author_icon": "http://flickr.com/icons/bobby.jpg",
                    "title": "Slack API Documentation",
                    "title_link": "https://api.slack.com/",
                    "text": "Optional text that appears within the attachment",
                    "fields": [
                        {
                            "title": "Priority",
                            "value": "High",
                            "short": false
                        }
                    ],
                    "image_url": "http://my-website.com/path/to/image.jpg",
                    "thumb_url": "http://example.com/path/to/thumb.png"
                }
            ]
        }"##).unwrap();
        match message {
            Message::Standard { ts: _, channel: _, user: _, text: _, is_starred, pinned_to: _, reactions: _, edited: _, attachments } => {
                assert_eq!(is_starred, Some(false));
                assert_eq!(attachments.unwrap()[0].color.as_ref().unwrap(), "#36a64f");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_bot_message() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "bot_message",
            "ts": "1358877455.000010",
            "text": "Pushing is the answer",
            "bot_id": "BB12033",
            "username": "github",
            "icons": {
                "image_24": "http://some.url.com/test.png"
            }
        }"#).unwrap();
        match message {
            Message::BotMessage { ts, text, bot_id, username, icons: _ } => {
                assert_eq!(ts, "1358877455.000010");
                assert_eq!(text, "Pushing is the answer");
                assert_eq!(bot_id, "BB12033");
                assert_eq!(username.unwrap(), "github");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_me_message() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "me_message",
            "channel": "C2147483705",
            "user": "U2147483697",
            "text": "is doing that thing",
            "ts": "1355517523.000005"
        }"#).unwrap();
        match message {
            Message::MeMessage { ts, text, user, channel } => {
                assert_eq!(ts, "1355517523.000005");
                assert_eq!(text, "is doing that thing");
                assert_eq!(channel, "C2147483705");
                assert_eq!(user, "U2147483697");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_message_changed() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "message_changed",
            "hidden": true,
            "channel": "C2147483705",
            "ts": "1358878755.000001",
            "message": {
                "type": "message",
                "user": "U2147483697",
                "text": "Hello, world!",
                "ts": "1355517523.000005",
                "edited": {
                    "user": "U2147483697",
                    "ts": "1358878755.000001"
                }
            }
        }"#).unwrap();
        match message {
            Message::MessageChanged { hidden: _, channel, ts, message } => {
                assert_eq!(ts, "1358878755.000001");
                assert_eq!(channel, "C2147483705");
                match *message.clone() {
                    Message::Standard { ts: _, channel: _, user, text: _, is_starred: _, pinned_to: _, reactions: _, edited, attachments: _ } => {
                        assert_eq!(user.unwrap(), "U2147483697");
                        assert_eq!(edited.unwrap().user, "U2147483697")
                    },
                    _ => panic!("Message decoded into incorrect variant.")
                }
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_message_deleted() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "message_deleted",
            "hidden": true,
            "channel": "C2147483705",
            "ts": "1358878755.000001",
            "deleted_ts": "1358878749.000002"
        }"#).unwrap();
        match message {
            Message::MessageDeleted { hidden: _, channel: _, ts, deleted_ts } => {
                assert_eq!(ts, "1358878755.000001");
                assert_eq!(deleted_ts, "1358878749.000002");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_join() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_join",
            "ts": "1358877458.000011",
            "user": "U2147483828",
            "text": "<@U2147483828|cal> has joined the channel"
        }"#).unwrap();
        match message {
            Message::ChannelJoin { user: _, text, ts: _, inviter } => {
                assert_eq!(text, "<@U2147483828|cal> has joined the channel");
                assert_eq!(inviter, None);
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_leave() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_leave",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "text": "<@U2147483828|cal> has left the channel"
        }"#).unwrap();
        match message {
            Message::ChannelLeave { user: _, text, ts: _ } => {
                assert_eq!(text, "<@U2147483828|cal> has left the channel");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_topic() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_topic",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "topic": "hello world",
            "text": "<@U2147483828|cal> set the channel topic: hello world"
        }"#).unwrap();
        match message {
            Message::ChannelTopic { user: _, text, ts: _, topic } => {
                assert_eq!(topic, "hello world");
                assert_eq!(text, "<@U2147483828|cal> set the channel topic: hello world");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_purpose() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_purpose",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "purpose": "whatever",
            "text": "<@U2147483828|cal> set the channel purpose: whatever"
        }"#).unwrap();
        match message {
            Message::ChannelPurpose { user: _, text, ts: _, purpose } => {
                assert_eq!(purpose, "whatever");
                assert_eq!(text, "<@U2147483828|cal> set the channel purpose: whatever");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_name() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_name",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "old_name": "random",
            "name": "watercooler",
            "text": "<@U2147483828|cal> has renamed the channek from \"random\" to \"watercooler\""
        }"#).unwrap();
        match message {
            Message::ChannelName { ts: _, user: _, old_name, name, text: _ } => {
                assert_eq!(old_name, "random");
                assert_eq!(name, "watercooler");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_archive() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_archive",
            "ts": "1361482916.000003",
            "text": "<U1234|@cal> archived the channel",
            "user": "U1234",
            "members": ["U1234", "U5678"]
        }"#).unwrap();
        match message {
            Message::ChannelArchive { ts: _, text: _, user, members } => {
                assert_eq!(user, "U1234");
                assert_eq!(members[1], "U5678");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_channel_unarchive() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "channel_unarchive",
            "ts": "1361482916.000003",
            "text": "<U1234|@cal> un-archived the channel",
            "user": "U1234"
        }"#).unwrap();
        match message {
            Message::ChannelUnarchive { ts: _, text: _, user } => {
                assert_eq!(user, "U1234");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_join() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_join",
            "ts": "1358877458.000011",
            "user": "U2147483828",
            "text": "<@U2147483828|cal> has joined the group"
        }"#).unwrap();
        match message {
            Message::GroupJoin { user: _, text, ts: _, inviter } => {
                assert_eq!(text, "<@U2147483828|cal> has joined the group");
                assert_eq!(inviter, None);
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_leave() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_leave",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "text": "<@U2147483828|cal> has left the group"
        }"#).unwrap();
        match message {
            Message::GroupLeave { user: _, text, ts: _ } => {
                assert_eq!(text, "<@U2147483828|cal> has left the group");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_topic() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_topic",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "topic": "hello world",
            "text": "<@U2147483828|cal> set the group topic: hello world"
        }"#).unwrap();
        match message {
            Message::GroupTopic { user: _, text, ts: _, topic } => {
                assert_eq!(topic, "hello world");
                assert_eq!(text, "<@U2147483828|cal> set the group topic: hello world");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_purpose() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_purpose",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "purpose": "whatever",
            "text": "<@U2147483828|cal> set the group purpose: whatever"
        }"#).unwrap();
        match message {
            Message::GroupPurpose { user: _, text, ts: _, purpose } => {
                assert_eq!(purpose, "whatever");
                assert_eq!(text, "<@U2147483828|cal> set the group purpose: whatever");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_name() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_name",
            "ts": "1358877455.000010",
            "user": "U2147483828",
            "old_name": "random",
            "name": "watercooler",
            "text": "<@U2147483828|cal> has renamed the group from \"random\" to \"watercooler\""
        }"#).unwrap();
        match message {
            Message::GroupName { ts: _, user: _, old_name, name, text: _ } => {
                assert_eq!(old_name, "random");
                assert_eq!(name, "watercooler");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_archive() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_archive",
            "ts": "1361482916.000003",
            "text": "<U1234|@cal> archived the group",
            "user": "U1234",
            "members": ["U1234", "U5678"]
        }"#).unwrap();
        match message {
            Message::GroupArchive { ts: _, text: _, user, members } => {
                assert_eq!(user, "U1234");
                assert_eq!(members[1], "U5678");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_group_unarchive() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "group_unarchive",
            "ts": "1361482916.000003",
            "text": "<U1234|@cal> un-archived the group",
            "user": "U1234"
        }"#).unwrap();
        match message {
            Message::GroupUnarchive { ts: _, text: _, user } => {
                assert_eq!(user, "U1234");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_file_share() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "file_share",
            "ts": "1358877455.000010",
            "text": "<@cal> uploaded a file: <https:...7.png|7.png>",
            "file": {
                "id": "F12345678",
                "created": 1444929467,
                "timestamp": 1444929467,
                "name": "test_img.png",
                "title": "test_img",
                "mimetype": "image\/png",
                "filetype": "png",
                "pretty_type": "PNG",
                "user": "U12345678",
                "editable": false,
                "size": 16153,
                "mode": "hosted",
                "is_external": false,
                "external_type": "",
                "is_public": true,
                "public_url_shared": false,
                "display_as_bot": false,
                "username": "",
                "url": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/test_img.png",
                "url_download": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/download\/test_img.png",
                "url_private": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/test_img.png",
                "url_private_download": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/download\/test_img.png",
                "thumb_64": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_64.png",
                "thumb_80": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_80.png",
                "thumb_360": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_360.png",
                "thumb_360_w": 360,
                "thumb_360_h": 28,
                "thumb_480": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_480.png",
                "thumb_480_w": 480,
                "thumb_480_h": 37,
                "thumb_160": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_160.png",
                "thumb_720": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_720.png",
                "thumb_720_w": 720,
                "thumb_720_h": 56,
                "image_exif_rotation": 1,
                "original_w": 895,
                "original_h": 69,
                "permalink": "https:\/\/test-team.slack.com\/files\/testuser\/F12345678\/test_img.png",
                "permalink_public": "https:\/\/slack-files.com\/PUBLIC-TEST-GUID",
                "channels": [
                    "C12345678"
                ],
                "groups": [

                ],
                "ims": [

                ],
                "comments_count": 0
            },
            "user": "U2147483697",
            "upload": true
        }"#).unwrap();
        match message {
            Message::FileShare { ts: _, file, text, user: _, upload: _ } => {
                assert_eq!(text, "<@cal> uploaded a file: <https:...7.png|7.png>");
                assert_eq!(file.id, "F12345678");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_file_comment() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "file_comment",
            "ts": "1358877455.000010",
            "text": "<@cal> commented on a file: ...",
            "file": {
                "id": "F12345678",
                "created": 1444929467,
                "timestamp": 1444929467,
                "name": "test_img.png",
                "title": "test_img",
                "mimetype": "image\/png",
                "filetype": "png",
                "pretty_type": "PNG",
                "user": "U12345678",
                "editable": false,
                "size": 16153,
                "mode": "hosted",
                "is_external": false,
                "external_type": "",
                "is_public": true,
                "public_url_shared": false,
                "display_as_bot": false,
                "username": "",
                "url": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/test_img.png",
                "url_download": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/download\/test_img.png",
                "url_private": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/test_img.png",
                "url_private_download": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/download\/test_img.png",
                "thumb_64": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_64.png",
                "thumb_80": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_80.png",
                "thumb_360": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_360.png",
                "thumb_360_w": 360,
                "thumb_360_h": 28,
                "thumb_480": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_480.png",
                "thumb_480_w": 480,
                "thumb_480_h": 37,
                "thumb_160": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_160.png",
                "thumb_720": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_720.png",
                "thumb_720_w": 720,
                "thumb_720_h": 56,
                "image_exif_rotation": 1,
                "original_w": 895,
                "original_h": 69,
                "permalink": "https:\/\/test-team.slack.com\/files\/testuser\/F12345678\/test_img.png",
                "permalink_public": "https:\/\/slack-files.com\/PUBLIC-TEST-GUID",
                "channels": [
                    "C12345678"
                ],
                "groups": [

                ],
                "ims": [

                ],
                "comments_count": 0
            },
            "comment": {
                "id": "Fc12345678",
                "timestamp": 1356032811,
                "user": "U12345678",
                "comment": "This is a comment"
            }
        }"#).unwrap();
        match message {
            Message::FileComment { ts: _, file, text, comment } => {
                assert_eq!(text, "<@cal> commented on a file: ...");
                assert_eq!(file.id, "F12345678");
                assert_eq!(comment.id, "Fc12345678");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_file_mention() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "file_mention",
            "ts": "1358877455.000010",
            "text": "<@cal> mentioned a file: <https:...7.png|7.png>",
            "file": {
                "id": "F12345678",
                "created": 1444929467,
                "timestamp": 1444929467,
                "name": "test_img.png",
                "title": "test_img",
                "mimetype": "image\/png",
                "filetype": "png",
                "pretty_type": "PNG",
                "user": "U12345678",
                "editable": false,
                "size": 16153,
                "mode": "hosted",
                "is_external": false,
                "external_type": "",
                "is_public": true,
                "public_url_shared": false,
                "display_as_bot": false,
                "username": "",
                "url": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/test_img.png",
                "url_download": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/download\/test_img.png",
                "url_private": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/test_img.png",
                "url_private_download": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/download\/test_img.png",
                "thumb_64": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_64.png",
                "thumb_80": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_80.png",
                "thumb_360": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_360.png",
                "thumb_360_w": 360,
                "thumb_360_h": 28,
                "thumb_480": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_480.png",
                "thumb_480_w": 480,
                "thumb_480_h": 37,
                "thumb_160": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_160.png",
                "thumb_720": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_720.png",
                "thumb_720_w": 720,
                "thumb_720_h": 56,
                "image_exif_rotation": 1,
                "original_w": 895,
                "original_h": 69,
                "permalink": "https:\/\/test-team.slack.com\/files\/testuser\/F12345678\/test_img.png",
                "permalink_public": "https:\/\/slack-files.com\/PUBLIC-TEST-GUID",
                "channels": [
                    "C12345678"
                ],
                "groups": [

                ],
                "ims": [

                ],
                "comments_count": 0
            },
            "user": "U2147483697"
        }"#).unwrap();
        match message {
            Message::FileMention { ts: _, file, text, user: _ } => {
                assert_eq!(text, "<@cal> mentioned a file: <https:...7.png|7.png>");
                assert_eq!(file.id, "F12345678");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_pinned_item() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "pinned_item",
            "user": "U024BE7LH",
            "item_type": "F",
            "text": "<@U024BE7LH|cal> pinned their Image <https:...7.png|7.png> to this channel.",
            "item": {
                "type": "file",
                "file": {
                    "id": "F12345678",
                    "created": 1444929467,
                    "timestamp": 1444929467,
                    "name": "test_img.png",
                    "title": "test_img",
                    "mimetype": "image\/png",
                    "filetype": "png",
                    "pretty_type": "PNG",
                    "user": "U12345678",
                    "editable": false,
                    "size": 16153,
                    "mode": "hosted",
                    "is_external": false,
                    "external_type": "",
                    "is_public": true,
                    "public_url_shared": false,
                    "display_as_bot": false,
                    "username": "",
                    "url": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/test_img.png",
                    "url_download": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/download\/test_img.png",
                    "url_private": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/test_img.png",
                    "url_private_download": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/download\/test_img.png",
                    "thumb_64": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_64.png",
                    "thumb_80": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_80.png",
                    "thumb_360": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_360.png",
                    "thumb_360_w": 360,
                    "thumb_360_h": 28,
                    "thumb_480": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_480.png",
                    "thumb_480_w": 480,
                    "thumb_480_h": 37,
                    "thumb_160": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_160.png",
                    "thumb_720": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_720.png",
                    "thumb_720_w": 720,
                    "thumb_720_h": 56,
                    "image_exif_rotation": 1,
                    "original_w": 895,
                    "original_h": 69,
                    "permalink": "https:\/\/test-team.slack.com\/files\/testuser\/F12345678\/test_img.png",
                    "permalink_public": "https:\/\/slack-files.com\/PUBLIC-TEST-GUID",
                    "channels": [
                        "C12345678"
                    ],
                    "groups": [

                    ],
                    "ims": [

                    ],
                    "comments_count": 0
                }
            },
            "channel": "C02ELGNBH",
            "ts": "1360782804.083113"
        }"#).unwrap();
        match message {
            Message::PinnedItem { ts: _, user: _, item_type, text: _, item, channel: _, attachments: _ } => {
                assert_eq!(item_type, "F");
                match item.unwrap() {
                    Item::File { file } => {
                        assert_eq!(file.id, "F12345678");
                    },
                    _ => panic!("Item decoded into incorrect variant.")
                }
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_pinned_item_no_item() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "pinned_item",
            "user": "U12345678",
            "item_type": "C",
            "attachments": [
                {
                    "color": "D0D0D0",
                    "fallback": "Test fallback message",
                    "ts": "1445226476.000002",
                    "text": "test message",
                    "author_link": "test-team.slack.com/team/user123",
                    "author_icon": "https://secure.gravatar.com/avatar/PRIVATE_GUID.jpg",
                    "mrkdwn_in": ["text"]
                }
            ],
            "text": "<@U12345678|user123> pinned a message to this channel.",
            "channel": "C12345678",
            "ts": "1445226479.000003"
        }"#).unwrap();
        match message {
            Message::PinnedItem { ts: _, user: _, item_type: _, text: _, item, channel: _, attachments } => {
                assert!(item.is_none());
                assert_eq!(attachments.unwrap()[0].color.as_ref().unwrap(), "D0D0D0");
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }

    #[test]
    fn decode_unpinned_item() {
        let message: Message = json::decode(r#"{
            "type": "message",
            "subtype": "unpinned_item",
            "user": "USLACKBOT",
            "item_type": "G",
            "text": "<@U024BE7LH|cal> unpinned the message you pinned to the secretplans group.",
            "item": {
                "type": "file",
                "file": {
                    "id": "F12345678",
                    "created": 1444929467,
                    "timestamp": 1444929467,
                    "name": "test_img.png",
                    "title": "test_img",
                    "mimetype": "image\/png",
                    "filetype": "png",
                    "pretty_type": "PNG",
                    "user": "U12345678",
                    "editable": false,
                    "size": 16153,
                    "mode": "hosted",
                    "is_external": false,
                    "external_type": "",
                    "is_public": true,
                    "public_url_shared": false,
                    "display_as_bot": false,
                    "username": "",
                    "url": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/test_img.png",
                    "url_download": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/download\/test_img.png",
                    "url_private": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/test_img.png",
                    "url_private_download": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/download\/test_img.png",
                    "thumb_64": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_64.png",
                    "thumb_80": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_80.png",
                    "thumb_360": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_360.png",
                    "thumb_360_w": 360,
                    "thumb_360_h": 28,
                    "thumb_480": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_480.png",
                    "thumb_480_w": 480,
                    "thumb_480_h": 37,
                    "thumb_160": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_160.png",
                    "thumb_720": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_720.png",
                    "thumb_720_w": 720,
                    "thumb_720_h": 56,
                    "image_exif_rotation": 1,
                    "original_w": 895,
                    "original_h": 69,
                    "permalink": "https:\/\/test-team.slack.com\/files\/testuser\/F12345678\/test_img.png",
                    "permalink_public": "https:\/\/slack-files.com\/PUBLIC-TEST-GUID",
                    "channels": [
                        "C12345678"
                    ],
                    "groups": [

                    ],
                    "ims": [

                    ],
                    "comments_count": 0
                }
            },
            "channel": "G024BE91L",
            "ts": "1360782804.083113"
        }"#).unwrap();
        match message {
            Message::UnpinnedItem { ts: _, user: _, item_type, text: _, item, channel: _, attachments: _ } => {
                assert_eq!(item_type, "G");
                match item.unwrap() {
                    Item::File { file } => {
                        assert_eq!(file.id, "F12345678");
                    },
                    _ => panic!("Item decoded into incorrect variant.")
                }
            },
            _ => panic!("Message decoded into incorrect variant.")
        };
    }
}
