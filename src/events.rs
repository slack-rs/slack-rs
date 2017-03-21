//
// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use api::{Bot, Message, File, FileComment, Channel, User, MessageUnpinnedItem, MessagePinnedItem,
          stars, reactions};
use serde::{de, Deserialize, Deserializer};
use std::boxed::Box;

/// Represents Slack [rtm event](https://api.slack.com/rtm) types.
#[derive(Clone, Debug)]
pub enum Event {
    /// Represents the slack [`hello`](https://api.slack.com/events/hello) event.
    Hello,
    /// Represents the slack [`message`](https://api.slack.com/events/message)
    /// event.
    Message(Box<Message>),
    /// Represents the slack
    /// [`user_typing`](https://api.slack.com/events/user_typing) event.
    UserTyping { channel: String, user: String },
    /// Represents the slack
    /// [`channel_marked`](https://api.slack.com/events/channel_marked) event.
    ChannelMarked { channel: String, ts: String },
    /// Represents the slack
    /// [`channel_created`](https://api.slack.com/events/channel_created) event.
    ChannelCreated { channel: Box<Channel> },
    /// Represents the slack
    /// [`channel_joined`](https://api.slack.com/events/channel_joined) event.
    ChannelJoined { channel: Box<Channel> },
    /// Represents the slack
    /// [`channel_left`](https://api.slack.com/events/channel_left) event.
    ChannelLeft { channel: String },
    /// Represents the slack
    /// [`channel_deleted`](https://api.slack.com/events/channel_deleted) event.
    ChannelDeleted { channel: String },
    /// Represents the slack
    /// [`channel_rename`](https://api.slack.com/events/channel_rename) event.
    ChannelRename { channel: Box<Channel> },
    /// Represents the slack
    /// [`channel_archive`](https://api.slack.com/events/channel_archive) event.
    ChannelArchive { channel: String, user: String },
    /// Represents the slack
    /// [`channel_unarchive`](https://api.slack.com/events/channel_unarchive) event.
    ChannelUnArchive { channel: String, user: String },
    /// Represents the slack
    /// [`channel_history_changed`](https://api.slack.
    /// com/events/channel_history_changed) event.
    ChannelHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`im_created`](https://api.slack.com/events/im_created) event.
    ImCreated { user: String, channel: Box<Channel> },
    /// Represents the slack [`im_open`](https://api.slack.com/events/im_open)
    /// event.
    ImOpen { user: String, channel: String },
    /// Represents the slack [`im_close`](https://api.slack.com/events/im_close)
    /// event.
    ImClose { user: String, channel: String },
    /// Represents the slack [`im_marked`](https://api.slack.com/events/im_marked)
    /// event.
    ImMarked { channel: String, ts: String },
    /// Represents the slack
    /// [`im_history_changed`](https://api.slack.com/events/im_history_changed)
    /// event.
    ImHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`group_joined`](https://api.slack.com/events/group_joined) event.
    GroupJoined { channel: Box<Channel> },
    /// Represents the slack
    /// [`group_left`](https://api.slack.com/events/group_left) event.
    GroupLeft { channel: Box<Channel> },
    /// Represents the slack
    /// [`group_open`](https://api.slack.com/events/group_open) event.
    GroupOpen { user: String, channel: String },
    /// Represents the slack
    /// [`group_close`](https://api.slack.com/events/group_close) event.
    GroupClose { user: String, channel: String },
    /// Represents the slack
    /// [`group_archive`](https://api.slack.com/events/group_archive) event.
    GroupArchive { channel: String },
    /// Represents the slack
    /// [`group_unarchive`](https://api.slack.com/events/group_unarchive) event.
    GroupUnArchive { channel: String },
    /// Represents the slack
    /// [`group_rename`](https://api.slack.com/events/group_rename) event.
    GroupRename { channel: Box<Channel> },
    /// Represents the slack
    /// [`group_marked`](https://api.slack.com/events/group_marked) event.
    GroupMarked { channel: String, ts: String },
    /// Represents the slack
    /// [`group_history_changed`](https://api.slack.
    /// com/events/group_history_changed) event.
    GroupHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`file_created`](https://api.slack.com/events/file_created) event.
    FileCreated { file: Box<File> },
    /// Represents the slack
    /// [`file_shared`](https://api.slack.com/events/file_shared) event.
    FileShared { file: Box<File> },
    /// Represents the slack
    /// [`file_unshared`](https://api.slack.com/events/file_unshared) event.
    FileUnShared { file: Box<File> },
    /// Represents the slack
    /// [`file_public`](https://api.slack.com/events/file_public) event.
    FilePublic { file: Box<File> },
    /// Represents the slack
    /// [`file_private`](https://api.slack.com/events/file_private) event.
    FilePrivate { file: String },
    /// Represents the slack
    /// [`file_change`](https://api.slack.com/events/file_change) event.
    FileChange { file: Box<File> },
    /// Represents the slack
    /// [`file_deleted`](https://api.slack.com/events/file_deleted) event.
    FileDeleted { file_id: String, event_ts: String },
    /// Represents the slack
    /// [`file_comment_added`](https://api.slack.com/events/file_comment_added)
    /// event.
    FileCommentAdded {
        file: Box<File>,
        comment: FileComment,
    },
    /// Represents the slack
    /// [`file_comment_edited`](https://api.slack.com/events/file_comment_edited)
    /// event.
    FileCommentEdited {
        file: Box<File>,
        comment: FileComment,
    },
    /// Represents the slack
    /// [`file_comment_deleted`](https://api.slack.com/events/file_comment_deleted)
    /// event.
    FileCommentDeleted { file: Box<File>, comment: String },
    /// Represents the slack [`pin_added`](https://api.slack.com/events/pin_added)
    /// event.
    PinAdded {
        user: String,
        channel_id: String,
        item: Box<MessagePinnedItem>,
        event_ts: String,
    },
    /// Represents the slack
    /// [`pin_removed`](https://api.slack.com/events/pin_removed) event.
    PinRemoved {
        user: String,
        channel_id: String,
        item: Box<MessageUnpinnedItem>,
        has_pins: bool,
        event_ts: String,
    },
    /// Represents the slack
    /// [`presence_change`](https://api.slack.com/events/presence_change) event.
    PresenceChange { user: String, presence: String },
    /// Represents the slack
    /// [`manual_presence_change`](https://api.slack.
    /// com/events/manual_presence_change) event.
    ManualPresenceChange { presence: String },
    /// Represents the slack
    /// [`pref_change`](https://api.slack.com/events/pref_change) event.
    PrefChange { name: String, value: String },
    /// Represents the slack
    /// [`user_change`](https://api.slack.com/events/user_change) event.
    UserChange { user: Box<User> },
    /// Represents the slack [`team_join`](https://api.slack.com/events/team_join)
    /// event.
    TeamJoin { user: Box<User> },
    /// Represents the slack
    /// [`star_added`](https://api.slack.com/events/star_added) event.
    StarAdded {
        user: String,
        item: Box<stars::ListResponseItem>,
        event_ts: String,
    },
    /// Represents the slack
    /// [`star_removed`](https://api.slack.com/events/star_removed) event.
    StarRemoved {
        user: String,
        item: Box<stars::ListResponseItem>,
        event_ts: String,
    },
    /// Represents the slack
    /// [`reaction_added`](https://api.slack.com/events/reaction_added) event.
    ReactionAdded {
        user: String,
        reaction: String,
        item: Box<reactions::ListResponseItem>,
        item_user: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`reaction_removed`](https://api.slack.com/events/reaction_removed) event.
    ReactionRemoved {
        user: String,
        reaction: String,
        item: Box<reactions::ListResponseItem>,
        item_user: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`emoji_changed`](https://api.slack.com/event/emoji_changed) event.
    EmojiChanged { event_ts: String },
    /// Represents the slack
    /// [`commands_changed`](https://api.slack.com/event/commands_changed) event.
    CommandsChanged { event_ts: String },
    /// Represents the slack
    /// [`team_plan_change`](https://api.slack.com/event/team_plan_change) event.
    TeamPlanChange { plan: String },
    /// Represents the slack
    /// [`team_pref_change`](https://api.slack.com/event/team_pref_change) event.
    TeamPrefChange { name: String, value: bool },
    /// Represents the slack
    /// [`team_rename`](https://api.slack.com/event/team_rename) event.
    TeamRename { name: String },
    /// Represents the slack
    /// [`team_domain_change`](https://api.slack.com/event/team_domain_change)
    /// event.
    TeamDomainChange { url: String, domain: String },
    /// Represents the slack
    /// [`email_domain_changeed`](https://api.slack.
    /// com/event/email_domain_changeed) event.
    EmailDomainChanged {
        email_domain: String,
        event_ts: String,
    },
    /// Represents the slack [`bot_added`](https://api.slack.com/event/bot_added)
    /// event.
    BotAdded { bot: Bot },
    /// Represents the slack
    /// [`bot_changed`](https://api.slack.com/event/bot_changed) event.
    BotChanged { bot: Bot },
    /// Represents the slack
    /// [`accounts_changed`](https://api.slack.com/event/accounts_changed) event.
    AccountsChanged,
    /// Represents the slack
    /// [`team_migration_started`](https://api.slack.
    /// com/event/team_migration_started) event.
    TeamMigrationStarted,
    /// Represents the slack
    /// [`reconnect_url`](https://api.slack.com/event/reconnect_url)
    /// event.
    ReconnectUrl,
    /// Represents a confirmation of a message sent
    MessageSent {
        reply_to: isize,
        ts: String,
        text: String,
    },
    /// Represents an error sending a message
    MessageError {
        reply_to: isize,
        code: isize,
        message: String,
    },
}

impl Deserialize for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct EventVisitor;

        impl de::Visitor for EventVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: de::MapVisitor
            {
                // assume first value provided is "type"
                let first: Option<(String, String)> = visitor.visit()?;
                if let Some((key, value)) = first {
                    match key.as_str() {
                        "type" => {
                            match value.as_str() {
                                "hello" => Ok(Event::Hello),
                                "message" => {
                                    let d = de::value::MapVisitorDeserializer::new(&mut visitor);
                                    Ok(Event::Message(Box::new(Message::deserialize(d)?)))
                                }
                                "accounts_changed" => Ok(Event::AccountsChanged),
                                "team_migration_started" => Ok(Event::TeamMigrationStarted),
                                "reconnect_url" => Ok(Event::ReconnectUrl),
                                "user_typing" => {
                                    let mut channel = None;
                                    let mut user = None;
                                    while let Some(key) = visitor.visit_key::<String>()? {
                                        match key.as_str() {
                                            "channel" => channel = Some(visitor.visit_value()?),
                                            "user" => user = Some(visitor.visit_value()?),
                                            _ => {}
                                        }
                                    }
                                    match (channel, user) {
                                        (Some(channel), Some(user)) => {
                                            Ok(Event::UserTyping { user, channel })
                                        }
                                        s => {
                                            Err(de::Error::custom(&format!("missing fields: {:?}",
                                                                           s)))
                                        }
                                    }
                                }
                                ty => {
                                    Err(de::Error::custom(&format!("Unknown Message type: {}", ty)))
                                }
                            }
                        }
                        _ => unimplemented!(),
                    }
                } else {
                    unimplemented!()
                }
            }
        }
        deserializer.deserialize_map(EventVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Message;
    use rustc_serialize::json;

    #[test]
    fn decode_short_standard_message() {
        let event: Event = json::decode(r#"{
            "type": "message",
            "ts": "1234567890.218332",
            "user": "U12345678",
            "text": "Hello world",
            "channel": "C12345678"
        }"#)
                .unwrap();
        match event {
            Event::Message(message) => {
                match message {
                    Message::Standard {
                        ts,
                        channel: _,
                        user,
                        text,
                        is_starred: _,
                        pinned_to: _,
                        reactions: _,
                        edited: _,
                        attachments: _,
                    } => {
                        assert_eq!(ts, "1234567890.218332");
                        assert_eq!(text.unwrap(), "Hello world");
                        assert_eq!(user.unwrap(), "U12345678");
                    }
                    _ => panic!("Message decoded into incorrect variant."),
                }
            }
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

    #[test]
    fn decode_sent_ok() {
        let event: Event = json::decode(r#"{
            "ok": true,
            "reply_to": 1,
            "ts": "1234567890.218332",
            "text": "Hello world"
        }"#)
                .unwrap();
        match event {
            Event::MessageSent { reply_to, ts, text } => {
                assert_eq!(reply_to, 1);
                assert_eq!(ts, "1234567890.218332");
                assert_eq!(text, "Hello world");
            }
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

    #[test]
    fn decode_sent_not_ok() {
        let event: Event = json::decode(r#"{
            "ok": false,
            "reply_to": 1,
            "error": {
                "code": 2,
                "msg": "message text is missing"
            }
        }"#)
                .unwrap();
        match event {
            Event::MessageError {
                reply_to,
                code,
                message,
            } => {
                assert_eq!(reply_to, 1);
                assert_eq!(code, 2);
                assert_eq!(message, "message text is missing");
            }
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

    #[test]
    fn decode_extended_standard_message() {
        let event: Event = json::decode(r##"{
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
        }"##)
                .unwrap();
        match event {
            Event::Message(message) => {
                match message {
                    Message::Standard {
                        ts: _,
                        channel: _,
                        user: _,
                        text: _,
                        is_starred,
                        pinned_to: _,
                        reactions: _,
                        edited: _,
                        attachments,
                    } => {
                        assert_eq!(is_starred, Some(false));
                        assert_eq!(attachments.unwrap()[0].color.as_ref().unwrap(), "#36a64f");
                    }
                    _ => panic!("Message decoded into incorrect variant."),
                }
            }
            _ => panic!("Event decoded into incorrect variant."),
        }
    }

}
