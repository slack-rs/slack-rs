//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add
pub fn add(client: &hyper::Client, token: &str, name: &str, file: Option<&str>, file_comment: Option<&str>, channel: Option<&str>, timestamp: Option<&str>) -> ApiResult<AddResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(channel) = channel {
        params.insert("channel", channel);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    make_authed_api_call(client, "reactions.add", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AddResponse;

/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get
pub fn get(client: &hyper::Client, token: &str, file: Option<&str>, file_comment: Option<&str>, channel: Option<&str>, timestamp: Option<&str>, full: Option<&str>) -> ApiResult<GetResponse> {
    let mut params = HashMap::new();
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(channel) = channel {
        params.insert("channel", channel);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    if let Some(full) = full {
        params.insert("full", full);
    }
    make_authed_api_call(client, "reactions.get", token, params)
}

// This is an Item as returned by `reactions.list`, but instead of being a nested object like all
// of the other endpoints is instead inlined at the top level.
pub type GetResponse = super::Item;

/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list
pub fn list(client: &hyper::Client, token: &str, user: Option<&str>, full: Option<&str>, count: Option<u32>, page: Option<u32>) -> ApiResult<ListResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    if let Some(user) = user {
        params.insert("user", user);
    }
    if let Some(full) = full {
        params.insert("full", full);
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "reactions.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub items: Vec<super::Item>,
    pub paging: super::Pagination
}

/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove
pub fn remove(client: &hyper::Client, token: &str, name: &str, file: Option<&str>, file_comment: Option<&str>, channel: Option<&str>, timestamp: Option<&str>) -> ApiResult<RemoveResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(channel) = channel {
        params.insert("channel", channel);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    make_authed_api_call(client, "reactions.remove", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RemoveResponse;

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;
    use super::super::Item;
    use super::super::Message;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = add(&client, "TEST_TOKEN", "thumbsup", None, None, Some("C1234567890"), Some("1234567890.123456"));
        assert!(result.is_err());
    }

    mock_slack_responder!(MockAddOkResponder, r#"{"ok": true}"#);

    #[test]
    fn add_ok_response() {
        let client = hyper::Client::with_connector(MockAddOkResponder::default());
        let result = add(&client, "TEST_TOKEN", "thumbsup", None, None, Some("C1234567890"), Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockGetOkResponder,
        r#"{
            "ok": true,
            "type": "message",
            "channel": "C1234567890",
            "message": {
                "type": "message",
                "channel": "C1234567890",
                "user": "U2147483697",
                "text": "Hello world",
                "ts": "1234567890.123456",
                "reactions": [
                    {
                        "name": "astonished",
                        "count": 3,
                        "users": [ "U1", "U2", "U3" ]
                    },
                    {
                        "name": "clock1",
                        "count": 2,
                        "users": [ "U1", "U2", "U3" ]
                    }
                ]
            }
        }"#
    );

    #[test]
    fn get_ok_response() {
        let client = hyper::Client::with_connector(MockGetOkResponder::default());
        let result = get(&client, "TEST_TOKEN", None, None, Some("C1234567890"), Some("1234567890.123456"), None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().clone() {
            Item::Message { channel: c, message: m } => {
                assert_eq!(c, "C1234567890");
                match *m.clone() {
                    Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions, edited: _, attachments: _ } => {
                        assert_eq!(text.unwrap(), "Hello world");
                        assert_eq!(reactions.unwrap()[0].name, "astonished");
                    },
                    _ => panic!("Message decoded into incorrect variant.")
                }
            },
            _ => panic!("Item decoded into incorrect variant.")
        };
    }

    mock_slack_responder!(MockListOkResponder,
        r#"{
            "ok": true,
            "items": [
                {
                    "type": "message",
                    "channel": "C1234567890",
                    "message": {
                        "type": "message",
                        "channel": "C1234567890",
                        "user": "U2147483697",
                        "text": "Hello world",
                        "ts": "1234567890.123456",
                        "reactions": [
                            {
                                "name": "astonished",
                                "count": 3,
                                "users": [ "U1", "U2", "U3" ]
                            },
                            {
                                "name": "clock1",
                                "count": 2,
                                "users": [ "U1", "U2", "U3" ]
                            }
                        ]
                    }
                },
                {
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
                        "comments_count": 0,
                        "reactions": [
                            {
                                "name": "thumbsup",
                                "count": 1,
                                "users": [ "U1" ]
                            }
                        ]
                    }
                }
            ],
            "paging": {
                "count": 100,
                "total": 5,
                "page": 1,
                "pages": 1
            }
        }"#
    );

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().items[1] {
            Item::File { file: ref f } => {
                assert_eq!(f.id, "F12345678");
                assert_eq!(f.reactions.as_ref().unwrap()[0].name, "thumbsup");
            },
            _ => panic!("Item decoded into incorrect variant.")
        };
    }

    mock_slack_responder!(MockRemoveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn remove_ok_response() {
        let client = hyper::Client::with_connector(MockRemoveOkResponder::default());
        let result = remove(&client, "TEST_TOKEN", "thumbsup", None, None, Some("C1234567890"), Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
