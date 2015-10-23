//! Search your team's files and messages.
//!
//! For more information, see [Slack's API documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

#[derive(Clone,Debug,RustcDecodable)]
pub struct SearchMatches<T> {
    pub total: u32,
    pub matches: Vec<T>,
    pub paging: super::Pagination
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MessageLink {
    pub user: String,
    pub username: String,
    pub ts: String,
    pub text: String
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SearchMessageChannel {
    pub id: String,
    pub name: String
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SearchMessage {
    pub user: String,
    pub username: String,
    pub ts: String,
    pub text: String,
    pub channel: SearchMessageChannel,
    pub permalink: String,
    pub previous: Option<MessageLink>,
    pub previous_2: Option<MessageLink>,
    pub next: Option<MessageLink>,
    pub next_2: Option<MessageLink>
}

/// Searches for messages and files matching a query.
///
/// Wraps https://api.slack.com/methods/search.all
pub fn all(client: &hyper::Client, token: &str, query: &str, sort: Option<&str>, sort_dir: Option<&str>, highlight: Option<bool>, count: Option<u32>, page: Option<u32>) -> ApiResult<AllResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    params.insert("query", query);
    if let Some(sort) = sort {
        params.insert("sort", sort);
    }
    if let Some(sort_dir) = sort_dir {
        params.insert("sort_dir", sort_dir);
    }
    if let Some(highlight) = highlight {
        params.insert("highlight", if highlight { "1" } else { "0" });
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "search.all", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AllResponse {
    pub query: String,
    pub messages: SearchMatches<SearchMessage>,
    pub files: SearchMatches<super::File>
}

/// Searches for files matching a query.
///
/// Wraps https://api.slack.com/methods/search.files
pub fn files(client: &hyper::Client, token: &str, query: &str, sort: Option<&str>, sort_dir: Option<&str>, highlight: Option<bool>, count: Option<u32>, page: Option<u32>) -> ApiResult<FilesResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    params.insert("query", query);
    if let Some(sort) = sort {
        params.insert("sort", sort);
    }
    if let Some(sort_dir) = sort_dir {
        params.insert("sort_dir", sort_dir);
    }
    if let Some(highlight) = highlight {
        params.insert("highlight", if highlight { "1" } else { "0" });
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "search.files", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct FilesResponse {
    pub query: String,
    pub files: SearchMatches<super::File>
}

/// Searches for messages matching a query.
///
/// Wraps https://api.slack.com/methods/search.messages
pub fn messages(client: &hyper::Client, token: &str, query: &str, sort: Option<&str>, sort_dir: Option<&str>, highlight: Option<bool>, count: Option<u32>, page: Option<u32>) -> ApiResult<MessagesResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    params.insert("query", query);
    if let Some(sort) = sort {
        params.insert("sort", sort);
    }
    if let Some(sort_dir) = sort_dir {
        params.insert("sort_dir", sort_dir);
    }
    if let Some(highlight) = highlight {
        params.insert("highlight", if highlight { "1" } else { "0" });
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "search.messages", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MessagesResponse {
    pub query: String,
    pub messages: SearchMatches<SearchMessage>
}

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = all(&client, "TEST_TOKEN", "pickleface", Some("timestamp"), Some("asc"), Some(true), Some(100), Some(1));
        assert!(result.is_err());
    }

    mock_slack_responder!(MockAllOkResponder, r#"
        {
            "ok": true,
            "query": "Best Pickles",
            "messages": {
                "total": 1,
                "matches": [
                    {
                        "type": "message",
                        "channel": {
                            "id": "C2147483753",
                            "name": "foo"
                        },
                        "user": "U2147483709",
                        "username": "johnnytest",
                        "ts": "1359414002.000003",
                        "text": "mention test: johnnyrodgers",
                        "permalink": "https:\/\/example.slack.com\/channels\/foo\/p1359414002000003",
                        "previous_2": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said before before",
                            "ts": "1359413987.000000",
                            "type": "message"
                        },
                        "previous": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said before",
                            "ts": "1359414001.000000",
                            "type": "message"
                        },
                        "next": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said after",
                            "ts": "1359414020.000000",
                            "type": "message"
                        },
                        "next_2": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said after after",
                            "ts": "1359414021.000000",
                            "type": "message"
                        }
                    }
                ],
                "paging": {
                    "count": 100,
                    "total": 15,
                    "page": 1,
                    "pages": 1
                }
            },
            "files": {
                "total": 1,
                "matches": [
                    {
                        "id" : "F2147483862",
                        "created" : 1356032811,
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
                        "ims": ["D12345"],
                        "num_stars": 7,
                        "is_starred": true,
                        "pinned_to": ["C024BE7LT"],
                        "reactions": [
                            {
                                "name": "astonished",
                                "count": 3,
                                "users": [ "U1", "U2", "U3" ]
                            },
                            {
                                "name": "facepalm",
                                "count": 1034,
                                "users": [ "U1", "U2", "U3", "U4", "U5" ]
                            }
                        ]
                    }
                ],
                "paging": {
                    "count": 100,
                    "total": 15,
                    "page": 1,
                    "pages": 1
                }
            }
        }
    "#);

    #[test]
    fn all_ok_response() {
        let client = hyper::Client::with_connector(MockAllOkResponder::default());
        let result = all(&client, "TEST_TOKEN", "pickleface", Some("timestamp"), Some("asc"), Some(true), Some(100), Some(1));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.messages.matches[0].user, "U2147483709");
        assert_eq!(result.files.matches[0].id, "F2147483862");
    }

    mock_slack_responder!(MockFilesOkResponder, r#"
        {
            "ok": true,
            "query": "Best Pickles",
            "files": {
                "total": 1,
                "matches": [
                    {
                        "id" : "F2147483862",
                        "created" : 1356032811,
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
                        "ims": ["D12345"],
                        "num_stars": 7,
                        "is_starred": true,
                        "pinned_to": ["C024BE7LT"],
                        "reactions": [
                            {
                                "name": "astonished",
                                "count": 3,
                                "users": [ "U1", "U2", "U3" ]
                            },
                            {
                                "name": "facepalm",
                                "count": 1034,
                                "users": [ "U1", "U2", "U3", "U4", "U5" ]
                            }
                        ]
                    }
                ],
                "paging": {
                    "count": 100,
                    "total": 15,
                    "page": 1,
                    "pages": 1
                }
            }
        }
    "#);

    #[test]
    fn files_ok_response() {
        let client = hyper::Client::with_connector(MockFilesOkResponder::default());
        let result = files(&client, "TEST_TOKEN", "pickleface", Some("timestamp"), Some("asc"), Some(true), Some(100), Some(1));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.files.matches[0].id, "F2147483862");
    }

    mock_slack_responder!(MockMessagesOkResponder, r#"
        {
            "ok": true,
            "query": "Best Pickles",
            "messages": {
                "total": 1,
                "matches": [
                    {
                        "type": "message",
                        "channel": {
                            "id": "C2147483753",
                            "name": "foo"
                        },
                        "user": "U2147483709",
                        "username": "johnnytest",
                        "ts": "1359414002.000003",
                        "text": "mention test: johnnyrodgers",
                        "permalink": "https:\/\/example.slack.com\/channels\/foo\/p1359414002000003",
                        "previous_2": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said before before",
                            "ts": "1359413987.000000",
                            "type": "message"
                        },
                        "previous": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said before",
                            "ts": "1359414001.000000",
                            "type": "message"
                        },
                        "next": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said after",
                            "ts": "1359414020.000000",
                            "type": "message"
                        },
                        "next_2": {
                            "user": "U2147483709",
                            "username": "johnnytest",
                            "text": "This was said after after",
                            "ts": "1359414021.000000",
                            "type": "message"
                        }
                    }
                ],
                "paging": {
                    "count": 100,
                    "total": 15,
                    "page": 1,
                    "pages": 1
                }
            }
        }
    "#);

    #[test]
    fn messages_ok_response() {
        let client = hyper::Client::with_connector(MockMessagesOkResponder::default());
        let result = messages(&client, "TEST_TOKEN", "pickleface", Some("timestamp"), Some("asc"), Some(true), Some(100), Some(1));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().messages.matches[0].user, "U2147483709");
    }
}
