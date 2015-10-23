//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add
pub fn add(client: &hyper::Client, token: &str, file: Option<&str>, file_comment: Option<&str>, channel: Option<&str>, timestamp: Option<&str>) -> ApiResult<AddResponse> {
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
    make_authed_api_call(client, "stars.add", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AddResponse;

/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list
pub fn list(client: &hyper::Client, token: &str, user: Option<&str>, count: Option<u32>, page: Option<u32>) -> ApiResult<ListResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    if let Some(user) = user {
        params.insert("user", user);
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "stars.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub items: Vec<super::StarredItem>,
    pub paging: super::Pagination
}

/// Removes a star from an item.
///
/// Wraps https://api.slack.com/methods/stars.remove
pub fn remove(client: &hyper::Client, token: &str, file: Option<&str>, file_comment: Option<&str>, channel: Option<&str>, timestamp: Option<&str>) -> ApiResult<RemoveResponse> {
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
    make_authed_api_call(client, "stars.remove", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RemoveResponse;

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;
    use super::super::StarredItem;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = add(&client, "TEST_TOKEN", None, None, Some("TEST_CHANNEL"), Some("1234567890.123456"));
        assert!(result.is_err());
    }

    mock_slack_responder!(MockAddOkResponder, r#"{"ok": true}"#);

    #[test]
    fn add_ok_response() {
        let client = hyper::Client::with_connector(MockAddOkResponder::default());
        let result = add(&client, "TEST_TOKEN", None, None, Some("TEST_CHANNEL"), Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockListOkResponder, r#"
        {
            "ok": true,
            "items": [
                {
                    "type": "message",
                    "channel": "C2147483705",
                    "message": {
                        "ts": "12345",
                        "user": "123",
                        "text": "something"
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
                    "type": "channel",
                    "channel": "C2147483705"
                }
            ],
            "paging": {
                "count": 100,
                "total": 15,
                "page": 1,
                "pages": 1
            }
        }
    "#);

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN", None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().items[2] {
            StarredItem::Channel { channel: ref c } => assert_eq!(c, "C2147483705"),
            _ => panic!("Expected Channel")
        }
    }

    mock_slack_responder!(MockRemoveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn remove_ok_response() {
        let client = hyper::Client::with_connector(MockRemoveOkResponder::default());
        let result = remove(&client, "TEST_TOKEN", None, None, Some("TEST_CHANNEL"), Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
