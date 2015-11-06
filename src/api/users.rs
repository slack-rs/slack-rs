//! Get info on members of your Slack team.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence
pub fn get_presence(client: &hyper::Client, token: &str, user: &str) -> ApiResult<GetPresenceResponse> {
    let mut params = HashMap::new();
    params.insert("user", user);
    make_authed_api_call(client, "users.getPresence", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct GetPresenceResponse {
    pub presence: String,
    pub online: Option<bool>,
    pub auto_away: Option<bool>,
    pub manual_away: Option<bool>,
    pub connection_count: Option<u32>,
    pub last_activity: Option<u32>,
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info
pub fn info(client: &hyper::Client, token: &str, user: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("user", user);
    make_authed_api_call(client, "users.info", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub user: super::User,
}

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list
pub fn list(client: &hyper::Client, token: &str, presence: Option<bool>) -> ApiResult<ListResponse> {
    let mut params = HashMap::new();
    if let Some(presence) = presence {
        params.insert("presence",
                      if presence {
                          "1"
                      } else {
                          "0"
                      });
    }
    make_authed_api_call(client, "users.list", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub members: Vec<super::User>,
}

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive
pub fn set_active(client: &hyper::Client, token: &str) -> ApiResult<SetActiveResponse> {
    make_authed_api_call(client, "users.setActive", token, HashMap::new())
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetActiveResponse;

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence
pub fn set_presence(client: &hyper::Client, token: &str, presence: &str) -> ApiResult<SetPresenceResponse> {
    let mut params = HashMap::new();
    params.insert("presence", presence);
    make_authed_api_call(client, "users.setPresence", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetPresenceResponse;

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = get_presence(&client, "TEST_TOKEN", "U12345678");
        assert!(result.is_err());
    }

    mock_slack_responder!(MockGetPresenceOkResponder, r#"
        {
            "ok": true,
            "presence": "active"
        }
    "#);

    #[test]
    fn get_presence_ok_response() {
        let client = hyper::Client::with_connector(MockGetPresenceOkResponder::default());
        let result = get_presence(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().presence, "active");
    }

    mock_slack_responder!(MockGetPresenceAuthedUserOkResponder, r#"
        {
            "ok": true,
            "presence": "active",
            "online": true,
            "auto_away": false,
            "manual_away": false,
            "connection_count": 1,
            "last_activity": 1419027078
        }
    "#);

    #[test]
    fn get_presence_authed_user_ok_response() {
        let client = hyper::Client::with_connector(MockGetPresenceAuthedUserOkResponder::default());
        let result = get_presence(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.presence, "active");
        assert_eq!(result.last_activity.unwrap(), 1419027078);
    }

    mock_slack_responder!(MockInfoOkResponder, r#"
        {
            "ok": true,
            "user": {
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
                "has_2fa": true,
                "has_files": true
            }
        }
    "#);

    #[test]
    fn info_ok_response() {
        let client = hyper::Client::with_connector(MockInfoOkResponder::default());
        let result = info(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.user.id, "U023BECGF");
        assert_eq!(result.user.profile.email.as_ref().unwrap(),
                   "bobby@slack.com");
    }

    mock_slack_responder!(MockListOkResponder, r#"
        {
            "ok": true,
            "members": [
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
                    "has_2fa": true,
                    "has_files": true
                },
                {
                    "id": "U12345678",
                    "name": "alice",
                    "deleted": false,
                    "color": "9f69e7",
                    "profile": {
                        "first_name": "Alice",
                        "last_name": "Aardvark",
                        "real_name": "Alice Aardvark",
                        "email": "alice@slack.com",
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
                    "has_2fa": true,
                    "has_files": true
                }
            ]
        }
    "#);

    #[test]
    fn list_ok_response() {
        let client = hyper::Client::with_connector(MockListOkResponder::default());
        let result = list(&client, "TEST_TOKEN", None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.members[1].id, "U12345678");
        assert_eq!(result.members[0].profile.email.as_ref().unwrap(),
                   "bobby@slack.com");
    }

    mock_slack_responder!(MockSetActiveOkResponder, r#"{"ok": true}"#);

    #[test]
    fn set_active_ok_response() {
        let client = hyper::Client::with_connector(MockSetActiveOkResponder::default());
        let result = set_active(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    mock_slack_responder!(MockSetPresenceOkResponder, r#"{"ok": true}"#);

    #[test]
    fn set_presence_ok_response() {
        let client = hyper::Client::with_connector(MockSetPresenceOkResponder::default());
        let result = set_presence(&client, "TEST_TOKEN", "active");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
