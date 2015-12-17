//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Pins an item to a channel.
///
/// Wraps https://api.slack.com/methods/pins.add
pub fn add(client: &hyper::Client,
           token: &str,
           channel: &str,
           file: Option<&str>,
           file_comment: Option<&str>,
           timestamp: Option<&str>)
           -> ApiResult<AddResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    make_authed_api_call(client, "pins.add", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AddResponse;

/// Lists items pinned to a channel.
///
/// Wraps https://api.slack.com/methods/pins.list
pub fn list(client: &hyper::Client, token: &str, channel: &str) -> ApiResult<ListResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    make_authed_api_call(client, "pins.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub items: Vec<super::Item>,
}

/// Un-pins an item from a channel.
///
/// Wraps https://api.slack.com/methods/pins.remove
pub fn remove(client: &hyper::Client,
              token: &str,
              channel: &str,
              file: Option<&str>,
              file_comment: Option<&str>,
              timestamp: Option<&str>)
              -> ApiResult<RemoveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    make_authed_api_call(client, "pins.remove", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RemoveResponse;

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;
    use super::super::Item;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = list(&client, "TEST_TOKEN", "TEST_CHANNEL");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockAddOkResponder, r#"{"ok": true}"#);

    #[test]
    fn add_ok_response() {
        let client = hyper::Client::with_connector(MockAddOkResponder::default());
        let result = add(&client,
                         "TEST_TOKEN",
                         "TEST_CHANNEL",
                         None,
                         None,
                         Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockListOkResponder, r#"{
        "ok": true,
        "items": [
            {
                "type": "message",
                "channel": "C2147483705",
                "message": {
                    "type": "message",
                    "user": "U024BE7LH",
                    "text": "lol",
                    "ts": "1444078138.000084"
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
                    "comments_count": 0
                }
            },
            {
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
            }
        ]
    }"#);

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().items[0] {
            Item::Message { channel: ref c, message: _ } => assert_eq!(c, "C2147483705"),
            _ => panic!("Incorrect item type. Expected message."),
        }
    }

    mock_slack_responder!(MockRemoveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn remove_ok_response() {
        let client = hyper::Client::with_connector(MockRemoveOkResponder::default());
        let result = remove(&client,
                            "TEST_TOKEN",
                            "TEST_CHANNEL",
                            None,
                            None,
                            Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
