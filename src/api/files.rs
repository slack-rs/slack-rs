//! Get info on files uploaded to Slack, upload new files to Slack.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Deletes a file.
///
/// Wraps https://api.slack.com/methods/files.delete
pub fn delete(client: &hyper::Client, token: &str, file: &str) -> ApiResult<DeleteResponse> {
    let mut params = HashMap::new();
    params.insert("file", file);
    make_authed_api_call(client, "files.delete", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct DeleteResponse;

/// Gets information about a team file.
///
/// Wraps https://api.slack.com/methods/files.info
pub fn info(client: &hyper::Client, token: &str, file: &str, count: Option<u32>, page: Option<u32>) -> ApiResult<InfoResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    params.insert("file", file);
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "files.info", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub file: super::File,
    pub comments: Vec<super::Comment>,
    pub paging: super::Pagination
}

/// Lists & filters team files.
///
/// Wraps https://api.slack.com/methods/files.list
pub fn list(client: &hyper::Client, token: &str, user: Option<&str>, ts_from: Option<&str>, ts_to: Option<&str>, types: Option<&str>, count: Option<u32>, page: Option<u32>) -> ApiResult<ListResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    if let Some(user) = user {
        params.insert("user", user);
    }
    if let Some(ts_from) = ts_from {
        params.insert("ts_from", ts_from);
    }
    if let Some(ts_to) = ts_to {
        params.insert("ts_to", ts_to);
    }
    if let Some(types) = types {
        params.insert("types", types);
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "files.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub files: Vec<super::File>,
    pub paging: super::Pagination
}

// /// Uploads or creates a file.
// ///
// /// Wraps https://api.slack.com/methods/files.upload
// #[allow(unused_variables)]
// pub fn upload(client: &hyper::Client, token: &str, file: Option<Vec<u8>>, content: Option<&str>, filetype: Option<&str>, filename: Option<&str>, title: Option<&str>, initial_comment: Option<&str>, channels: Option<&str>) -> ApiResult<UploadResponse> {
//     Err(::error::Error::Api(String::from("Currently unsupported.")))
// }
//
// #[derive(Clone,Debug,RustcDecodable)]
// pub struct UploadResponse {
//     pub file: super::File
// }

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = delete(&client, "TEST_TOKEN", "F12345678");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockDeleteOkResponder, r#"{"ok": true}"#);

    #[test]
    fn delete_ok_response() {
        let client = hyper::Client::with_connector(MockDeleteOkResponder::default());
        let result = delete(&client, "TEST_TOKEN", "F12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockInfoOkResponder, r#"{
        "ok": true,
        "file": {
            "id" : "F2147483862",
            "timestamp" : 1356032811,

            "name" : "file.htm",
            "title" : "My HTML file",
            "mimetype" : "text\/plain",
            "filetype" : "text",
            "pretty_type": "Text",
            "user" : "U2147483697",

            "mode" : "hosted",
            "editable" : true,
            "is_external": false,
            "external_type": "",

            "size" : 12345,

            "url": "https:\/\/slack-files.com\/files-pub\/T024BE7LD-F024BERPE-09acb6\/1.png",
            "url_download": "https:\/\/slack-files.com\/files-pub\/T024BE7LD-F024BERPE-09acb6\/download\/1.png",
            "url_private": "https:\/\/slack.com\/files-pri\/T024BE7LD-F024BERPE\/1.png",
            "url_private_download": "https:\/\/slack.com\/files-pri\/T024BE7LD-F024BERPE\/download\/1.png",

            "thumb_64": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_64.png",
            "thumb_80": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_80.png",
            "thumb_360": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_360.png",
            "thumb_360_gif": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_360.gif",
            "thumb_360_w": 100,
            "thumb_360_h": 100,

            "permalink" : "https:\/\/tinyspeck.slack.com\/files\/cal\/F024BERPE\/1.png",
            "edit_link" : "https:\/\/tinyspeck.slack.com\/files\/cal\/F024BERPE\/1.png/edit",
            "preview" : "&lt;!DOCTYPE html&gt;\n&lt;html&gt;\n&lt;meta charset='utf-8'&gt;",
            "preview_highlight" : "&lt;div class=\"sssh-code\"&gt;&lt;div class=\"sssh-line\"&gt;&lt;pre&gt;&lt;!DOCTYPE html...",
            "lines" : 123,
            "lines_more": 118,

            "is_public": true,
            "public_url_shared": false,
            "channels": ["C024BE7LT"],
            "groups": ["G12345"],
            "initial_comment": {
                "id": "Fc027BN9L9",
                "timestamp": 1356032811,
                "user": "U2147483697",
                "comment": "This is a comment"
            },
            "num_stars": 7,
            "is_starred": true
        },
        "comments": [
            {
                "id": "Fc027BN9L9",
                "timestamp": 1356032811,
                "user": "U2147483697",
                "comment": "This is a comment"
            }
        ],
        "paging": {
            "count": 100,
            "total": 2,
            "page": 1,
            "pages": 0
        }
    }"#);

    #[test]
    fn info_ok_response() {
        let client = hyper::Client::with_connector(MockInfoOkResponder::default());
        let result = info(&client, "TEST_TOKEN", "F12345678", None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().file.id, "F2147483862");
    }

    mock_slack_responder!(MockListOkResponder, r#"{
        "ok": true,
        "files": [
            {
                "id" : "F2147483862",
                "timestamp" : 1356032811,

                "name" : "file.htm",
                "title" : "My HTML file",
                "mimetype" : "text\/plain",
                "filetype" : "text",
                "pretty_type": "Text",
                "user" : "U2147483697",

                "mode" : "hosted",
                "editable" : true,
                "is_external": false,
                "external_type": "",

                "size" : 12345,

                "url": "https:\/\/slack-files.com\/files-pub\/T024BE7LD-F024BERPE-09acb6\/1.png",
                "url_download": "https:\/\/slack-files.com\/files-pub\/T024BE7LD-F024BERPE-09acb6\/download\/1.png",
                "url_private": "https:\/\/slack.com\/files-pri\/T024BE7LD-F024BERPE\/1.png",
                "url_private_download": "https:\/\/slack.com\/files-pri\/T024BE7LD-F024BERPE\/download\/1.png",

                "thumb_64": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_64.png",
                "thumb_80": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_80.png",
                "thumb_360": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_360.png",
                "thumb_360_gif": "https:\/\/slack-files.com\/files-tmb\/T024BE7LD-F024BERPE-c66246\/1_360.gif",
                "thumb_360_w": 100,
                "thumb_360_h": 100,

                "permalink" : "https:\/\/tinyspeck.slack.com\/files\/cal\/F024BERPE\/1.png",
                "edit_link" : "https:\/\/tinyspeck.slack.com\/files\/cal\/F024BERPE\/1.png/edit",
                "preview" : "&lt;!DOCTYPE html&gt;\n&lt;html&gt;\n&lt;meta charset='utf-8'&gt;",
                "preview_highlight" : "&lt;div class=\"sssh-code\"&gt;&lt;div class=\"sssh-line\"&gt;&lt;pre&gt;&lt;!DOCTYPE html...",
                "lines" : 123,
                "lines_more": 118,

                "is_public": true,
                "public_url_shared": false,
                "channels": ["C024BE7LT"],
                "groups": ["G12345"],
                "initial_comment": {
                    "id": "Fc027BN9L9",
                    "timestamp": 1356032811,
                    "user": "U2147483697",
                    "comment": "This is a comment"
                },
                "num_stars": 7,
                "is_starred": true
            },
            {
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
        ],
        "paging": {
            "count": 100,
            "total": 295,
            "page": 1,
            "pages": 3
        }
    }"#);

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN", None, None, None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().files[0].id, "F2147483862");
    }
}
