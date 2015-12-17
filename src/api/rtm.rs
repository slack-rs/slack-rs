//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.start
pub fn start(client: &hyper::Client, token: &str, simple_latest: Option<bool>, no_unreads: Option<bool>) -> ApiResult<StartResponse> {
    let mut params = HashMap::new();
    if let Some(simple_latest) = simple_latest {
        params.insert("simple_latest",
                      if simple_latest {
                          "1"
                      } else {
                          "0"
                      });
    }
    if let Some(no_unreads) = no_unreads {
        params.insert("no_unreads",
                      if no_unreads {
                          "1"
                      } else {
                          "0"
                      });
    }
    make_authed_api_call(client, "rtm.start", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct Bot {
    pub id: String,
    pub deleted: Option<bool>,
    pub name: String,
    pub icons: Option<HashMap<String, String>>,
}

// We've left out the prefs field for now
#[derive(Clone,Debug,RustcDecodable)]
pub struct SelfData {
    pub id: String,
    pub name: String,
    pub created: i64,
    pub manual_presence: String,
}

#[derive(Clone,Debug)]
pub struct StartResponse {
    pub url: String,
    pub self_data: SelfData,
    pub team: super::Team,
    pub users: Vec<super::User>,
    pub channels: Vec<super::Channel>,
    pub groups: Vec<super::Group>,
    pub ims: Vec<super::Im>,
    pub bots: Vec<Bot>,
}

// This is an ugly hack, we have to compile with --pretty expanded and fix up
// self to map to self_data.
// An alternative would be using serde, but it won't do what we need on stable.
impl ::rustc_serialize::Decodable for StartResponse {
    fn decode<__D: ::rustc_serialize::Decoder>(__arg_0: &mut __D) -> ::std::result::Result<StartResponse, __D::Error> {
        __arg_0.read_struct("StartResponse", 8usize, |_d| -> _ {
            ::std::result::Result::Ok(StartResponse {
                url: match _d.read_struct_field("url", 0usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                self_data: match _d.read_struct_field("self", 1usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                team: match _d.read_struct_field("team", 2usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                users: match _d.read_struct_field("users", 3usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                channels: match _d.read_struct_field("channels", 4usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                groups: match _d.read_struct_field("groups", 5usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                ims: match _d.read_struct_field("ims", 6usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
                bots: match _d.read_struct_field("bots", 7usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => return ::std::result::Result::Err(__try_var),
                },
            })
        })
    }
}


#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = start(&client, "TEST_TOKEN", None, None);
        assert!(result.is_err());
    }

    mock_slack_responder!(MockStartOkResponder, r#"
        {
            "ok": true,
            "url": "wss:\/\/ms9.slack-msgs.com\/websocket\/7I5yBpcvk",
            "self": {
                "id": "U023BECGF",
                "name": "bobby",
                "prefs": {},
                "created": 1402463766,
                "manual_presence": "active"
            },
            "team": {
                "id": "T024BE7LD",
                "name": "Example Team",
                "email_domain": "",
                "domain": "example",
                "msg_edit_window_mins": -1,
                "over_storage_limit": false,
                "prefs": {},
                "plan": "std"
            },
            "users": [
                {
                    "id": "U023BECGF",
                    "name": "bobby",
                    "deleted": false,
                    "color": "9f69e7",
                    "profile": {
                        "first_name": "Bobby",
                        "last_name": "Tables",
                        "real_name": "Bobby Tables",
                        "email": "bobby@slack.com",
                        "skype": "my-skype-name",
                        "phone": "+1 (123) 456 7890",
                        "image_24": "https:\/\/...",
                        "image_32": "https:\/\/...",
                        "image_48": "https:\/\/...",
                        "image_72": "https:\/\/...",
                        "image_192": "https:\/\/..."
                    },
                    "is_admin": true,
                    "is_owner": true,
                    "is_primary_owner": true,
                    "is_restricted": false,
                    "is_ultra_restricted": false,
                    "has_2fa": false,
                    "two_factor_type": "sms",
                    "has_files": true
                }
            ],
            "channels": [
                {
                    "id": "C024BE91L",
                    "name": "fun",
                    "is_channel": true,
                    "created": 1360782804,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_general": false,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "Fun times",
                        "creator": "U024BE7LV",
                        "last_set": 1369677212
                    },
                    "purpose": {
                        "value": "This channel is for fun",
                        "creator": "U024BE7LH",
                        "last_set": 1360782804
                    },
                    "is_member": true,
                    "last_read": "1401383885.000061",
                    "latest": "1401383885.000061",
                    "unread_count": 0,
                    "unread_count_display": 0
                }
            ],
            "groups": [
                {
                    "id": "G024BE91L",
                    "name": "secretplans",
                    "is_group": true,
                    "created": 1360782804,
                    "creator": "U024BE7LH",
                    "is_archived": false,
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
                    },
                    "last_read": "1401383885.000061",
                    "latest": "1401383885.000061",
                    "unread_count": 0,
                    "unread_count_display": 0
                }
            ],
            "ims": [
                {
                    "id": "D024BFF1M",
                    "is_im": true,
                    "user": "U024BE7LH",
                    "created": 1360782804,
                    "is_user_deleted": false
                }
            ],
            "bots": [
                {
                    "id": "B87654321",
                    "deleted": false,
                    "name": "gdrive"
                },
                {
                    "id": "B12345678",
                    "deleted": false,
                    "name": "bot",
                    "icons": {
                        "image_48": "https:\/\/slack.global.ssl.fastly.net\/BOT_ID\/img\/services\/bots_48.png"
                    }
                }
            ]
        }
    "#);

    #[test]
    fn start_ok_response() {
        let client = hyper::Client::with_connector(MockStartOkResponder::default());
        let result = start(&client, "TEST_TOKEN", Some(true), None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.url, "wss://ms9.slack-msgs.com/websocket/7I5yBpcvk");
        assert_eq!(result.self_data.id, "U023BECGF");
        assert_eq!(result.team.id, "T024BE7LD");
        assert_eq!(result.users[0].id, "U023BECGF");
        assert_eq!(result.channels[0].id, "C024BE91L");
        assert_eq!(result.groups[0].id, "G024BE91L");
        assert_eq!(result.ims[0].id, "D024BFF1M");
        assert_eq!(result.bots[0].id, "B87654321");
    }
}
