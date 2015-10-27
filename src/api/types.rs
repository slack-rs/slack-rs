use rustc_serialize::{Decodable,Decoder};

/// The `Reaction` object as found in the [`File`](https://api.slack.com/types/file) type and some
/// [`Message`](https://api.slack.com/events/message) responses.
#[derive(Clone,Debug,RustcDecodable)]
pub struct Reaction {
    pub name: String,
    pub count: u32,
    pub users: Vec<String>
}

/// The `Comment` object as found in the [`File`](https://api.slack.com/types/file) type and the
/// [`files.info`](https://api.slack.com/methods/files.info) response.
#[derive(Clone,Debug,RustcDecodable)]
pub struct Comment {
    pub id: String,
    pub timestamp: u32,
    pub user: String,
    pub comment: String,
    pub reactions: Option<Vec<Reaction>>
}

/// The Slack [`File`](https://api.slack.com/types/file) type.
#[derive(Clone,Debug,RustcDecodable)]
pub struct File {
    pub id: String,
    pub created: Option<u32>,
    pub timestamp: Option<u32>,
    pub name: Option<String>,
    pub title: String,
    pub mimetype: String,
    pub filetype: String,
    pub pretty_type: String,
    pub user: String,
    pub mode: String,
    pub editable: bool,
    pub is_external: bool,
    pub external_type: String,
    pub size: u32,
    pub url: String,
    pub url_download: Option<String>,
    pub url_private: String,
    pub url_private_download: String,
    pub thumb_64: String,
    pub thumb_80: String,
    pub thumb_360: String,
    pub thumb_360_gif: Option<String>,
    pub thumb_360_w: u32,
    pub thumb_360_h: u32,
    pub permalink: String,
    pub edit_link: Option<String>,
    pub preview: Option<String>,
    pub preview_highlight: Option<String>,
    pub lines: Option<u32>,
    pub lines_more: Option<u32>,
    pub is_public: bool,
    pub public_url_shared: bool,
    pub channels: Vec<String>,
    pub groups: Vec<String>,
    pub ims: Option<Vec<String>>,
    pub initial_comment: Option<Comment>,
    pub num_stars: Option<u32>,
    pub is_starred: Option<bool>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<Reaction>>
}

/// The `Paging` object as found in API endpoints that return pages of items.
#[derive(Clone,Debug,RustcDecodable)]
pub struct Pagination {
    pub count: u32,
    pub total: u32,
    pub page: u32,
    pub pages: u32
}

/// The Slack [`Channel`](https://api.slack.com/types/channel) type.
// Currently missing "latest" field
#[derive(Clone,Debug,RustcDecodable)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub created: i64,
    pub creator: String,
    pub is_archived: bool,
    pub is_general: bool,
    pub members: Option<Vec<String>>,
    pub topic: Option<Topic>,
    pub purpose: Option<Purpose>,
    pub is_member: bool,
    pub last_read: Option<String>,
    pub unread_count: Option<i64>,
    pub unread_count_display: Option<i64>,
}

/// The Slack [`Group`](https://api.slack.com/types/group) type.
// Currently missing "latest" field
#[derive(Clone,Debug,RustcDecodable)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub is_group: bool,
    pub created: i64,
    pub creator:  String,
    pub is_archived:  bool,
    pub members: Option<Vec<String>>,
    pub topic: Option<Topic>,
    pub purpose: Option<Purpose>,
    pub last_read: Option<String>,
    pub unread_count: Option<i64>,
    pub unread_count_display: Option<i64>,
}

/// The Profile that belongs to a [`User`](https://api.slack.com/types/user).
#[derive(Clone,Debug,RustcDecodable)]
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub real_name: Option<String>,
    pub email: Option<String>,
    pub skype: Option<String>,
    pub phone: Option<String>,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String
}

/// The Slack [`User`](https://api.slack.com/types/user) type.
#[derive(Clone,Debug,RustcDecodable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub deleted: bool,
    pub color: Option<String>,
    pub profile: UserProfile,
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub has_2fa: Option<bool>,
    pub two_factor_type: Option<String>,
    pub has_files: Option<bool>
}

/// The `Team` object as found in the [`rtm.start`](https://api.slack.com/methods/rtm.start)
/// response.
// We've left out the prefs field for now
#[derive(Clone,Debug,RustcDecodable)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub email_domain: String,
    pub domain: String,
    pub msg_edit_window_mins: i64,
    pub over_storage_limit: bool,
    pub plan: String,
}

/// The `Topic` object as found in the [`Group`](https://api.slack.com/types/group) and
/// [`Channel`](https://api.slack.com/types/channel) types.
#[derive(Clone,Debug,RustcDecodable)]
pub struct Topic {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

/// The `Purpose` object as found in the [`Group`](https://api.slack.com/types/group) and
/// [`Channel`](https://api.slack.com/types/channel) types.
#[derive(Clone,Debug,RustcDecodable)]
pub struct Purpose {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

/// The Slack [`Im`](https://api.slack.com/types/im) type.
#[derive(Clone,Debug,RustcDecodable)]
pub struct Im {
    pub id: String,
    pub is_im: bool,
    pub user:  String,
    pub created: i64,
    pub is_user_deleted: Option<bool>,
}

/// A field within an [`Attachment`](https://api.slack.com/docs/attachments).
#[derive(Clone,Debug,RustcDecodable,RustcEncodable)]
pub struct AttachmentField {
    pub title: String,
    pub value: String,
    pub short: bool
}

/// The Slack [`Attachment`](https://api.slack.com/docs/attachments) object as found in
/// richly-formatted messages.
#[derive(Clone,Debug,RustcDecodable,RustcEncodable)]
pub struct Attachment {
    pub fallback: String,
    pub color: Option<String>,
    pub pretext: Option<String>,
    pub author_name: Option<String>,
    pub author_link: Option<String>,
    pub author_icon: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub text: String,
    pub fields: Option<Vec<AttachmentField>>,
    pub image_url: Option<String>,
    pub thumb_url: Option<String>
}

/// Represents a `Message`, `File` or `Comment` as returned by
/// [`pins.list`](https://api.slack.com/methods/pins.list),
/// [`reactions.list`](https://api.slack.com/methods/reactions.list), and
/// [`reactions.get`](https://api.slack.com/methods/reactions.get).
#[derive(Clone,Debug)]
pub enum Item {
    Message { channel: String, message: Box<super::Message> },
    File { file: File },
    FileComment { file: File, comment: Comment }
}

impl Decodable for Item {
    fn decode<D: Decoder>(d: &mut D) -> Result<Item, D::Error> {
        d.read_struct("item", 0, |d| {
            let ty: String = try!(d.read_struct_field("type", 0, |d| Decodable::decode(d)));
            if ty == "message" {
                Ok(Item::Message {
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    message: try!(d.read_struct_field("message", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file" {
                Ok(Item::File {
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file_comment" {
                Ok(Item::FileComment {
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    comment: try!(d.read_struct_field("comment", 0, |d| Decodable::decode(d)))
                })
            } else {
                Err(d.error(&format!("Unknown Item type: {}", ty)))
            }
        })
    }
}

/// Represents a starred item as returned by [`stars.list`](https://api.slack.com/methods/stars.list).
// The Message, File and FileComment variants are the same as the ones in `super::Item`. However,
// stars can be applied to channels, groups and ims as well, so we need a new enum to support those
// options.
#[derive(Clone,Debug)]
pub enum StarredItem {
    Message { channel: String, message: super::Message},
    File { file: super::File },
    FileComment { file: super::File, comment: super::Comment },
    Channel { channel: String },
    Group { group: String },
    Im { channel: String }
}

impl Decodable for StarredItem {
    fn decode<D: Decoder>(d: &mut D) -> Result<StarredItem, D::Error> {
        d.read_struct("item", 0, |d| {
            let ty: String = try!(d.read_struct_field("type", 0, |d| Decodable::decode(d)));
            if ty == "message" {
                Ok(StarredItem::Message {
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                    message: try!(d.read_struct_field("message", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file" {
                Ok(StarredItem::File {
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "file_comment" {
                Ok(StarredItem::FileComment {
                    file: try!(d.read_struct_field("file", 0, |d| Decodable::decode(d))),
                    comment: try!(d.read_struct_field("comment", 0, |d| Decodable::decode(d)))
                })
            } else if ty == "channel" {
                Ok(StarredItem::Channel {
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                })
            } else if ty == "group" {
                Ok(StarredItem::Group {
                    group: try!(d.read_struct_field("group", 0, |d| Decodable::decode(d))),
                })
            } else if ty == "im" {
                Ok(StarredItem::Im {
                    channel: try!(d.read_struct_field("channel", 0, |d| Decodable::decode(d))),
                })
            } else {
                Err(d.error(&format!("Unknown StarredItem type: {}", ty)))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Message;
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn item_message_decode() {
        let item: Item = json::decode(r#"{
            "type": "message",
            "channel": "C2147483705",
            "message": {
                "ts": "12345",
                "user": "123",
                "text": "something"
            }
        }"#).unwrap();
        match item {
            Item::Message { channel: c, message: m } => {
                assert_eq!(c, "C2147483705");
                match *m.clone() {
                    Message::Standard { ts: _, channel: _, user, text: _, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                        assert_eq!(user.unwrap(), "123")
                    },
                    _ => panic!("Message decoded into incorrect variant.")
                }
            },
            _ => panic!("Item decoded into incorrect variant.")
        };
    }

    #[test]
    fn item_file_decode() {
        let item: Item = json::decode(r#"{
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
        }"#).unwrap();
        match item {
            Item::File { file: f } => {
                assert_eq!(f.id, "F12345678");
            },
            _ => panic!("Item decoded into incorrect variant.")
        };
    }

    #[test]
    fn item_file_comment_decode() {
        let item: Item = json::decode(r#"{
            "type": "file_comment",
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
        match item {
            Item::FileComment { file: f, comment: c } => {
                assert_eq!(f.id, "F12345678");
                assert_eq!(c.id, "Fc12345678");
            },
            _ => panic!("Item decoded into incorrect variant.")
        };
    }

    #[test]
    fn starred_item_message_decode() {
        let item: StarredItem = json::decode(r#"{
            "type": "message",
            "channel": "C2147483705",
            "message": {
                "ts": "12345",
                "user": "123",
                "text": "something"
            }
        }"#).unwrap();
        match item {
            StarredItem::Message { channel: c, message: m } => {
                assert_eq!(c, "C2147483705");
                match m.clone() {
                    Message::Standard { ts: _, channel: _, user, text: _, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                        assert_eq!(user.unwrap(), "123");
                    },
                    _ => panic!("Message decoded into incorrect variant.")
                }
            },
            _ => panic!("StarredItem decoded into incorrect variant.")
        };
    }

    #[test]
    fn starred_item_file_decode() {
        let item: StarredItem = json::decode(r#"{
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
        }"#).unwrap();
        match item {
            StarredItem::File { file: f } => {
                assert_eq!(f.id, "F12345678");
            },
            _ => panic!("StarredItem decoded into incorrect variant.")
        };
    }

    #[test]
    fn starred_item_file_comment_decode() {
        let item: StarredItem = json::decode(r#"{
            "type": "file_comment",
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
        match item {
            StarredItem::FileComment { file: f, comment: c } => {
                assert_eq!(f.id, "F12345678");
                assert_eq!(c.id, "Fc12345678");
            },
            _ => panic!("StarredItem decoded into incorrect variant.")
        };
    }

    #[test]
    fn starred_item_channel_decode() {
        let item: StarredItem = json::decode(r#"{"type": "channel", "channel": "C12345678"}"#).unwrap();
        match item {
            StarredItem::Channel { channel: c } => {
                assert_eq!(c, "C12345678");
            },
            _ => panic!("StarredItem decoded into incorrect variant.")
        };
    }

    #[test]
    fn starred_item_group_decode() {
        let item: StarredItem = json::decode(r#"{"type": "group", "group": "G12345678"}"#).unwrap();
        match item {
            StarredItem::Group { group: g } => {
                assert_eq!(g, "G12345678");
            },
            _ => panic!("StarredItem decoded into incorrect variant.")
        };
    }

    #[test]
    fn starred_item_im_decode() {
        let item: StarredItem = json::decode(r#"{"type": "im", "channel": "D12345678"}"#).unwrap();
        match item {
            StarredItem::Im { channel: c } => {
                assert_eq!(c, "D12345678");
            },
            _ => panic!("StarredItem decoded into incorrect variant.")
        };
    }
}
