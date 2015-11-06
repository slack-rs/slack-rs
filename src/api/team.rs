//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;
use hyper;

use super::ApiResult;
use super::make_authed_api_call;

/// Gets the access logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.accessLogs
pub fn access_logs(client: &hyper::Client, token: &str, count: Option<u32>, page: Option<u32>) -> ApiResult<AccessLogsResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params: HashMap<&str, &str> = HashMap::new();
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    make_authed_api_call(client, "team.accessLogs", token, params)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct LoginInfo {
    pub user_id: String,
    pub username: String,
    pub date_first: u32,
    pub date_last: u32,
    pub count: u32,
    pub ip: String,
    pub user_agent: String,
    pub isp: String,
    pub country: String,
    pub region: String,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AccessLogsResponse {
    pub logins: Vec<LoginInfo>,
    pub paging: super::Pagination,
}

/// Gets information about the current team.
///
/// Wraps https://api.slack.com/methods/team.info
pub fn info(client: &hyper::Client, token: &str) -> ApiResult<InfoResponse> {
    make_authed_api_call(client, "team.info", token, HashMap::new())
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct IconInfo {
    pub image_34: String,
    pub image_44: String,
    pub image_68: String,
    pub image_88: String,
    pub image_102: String,
    pub image_132: String,
    pub image_default: bool,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub email_domain: String,
    pub icon: IconInfo,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub team: TeamInfo,
}

#[cfg(test)]
mod tests {
    use hyper;
    use super::*;

    mock_slack_responder!(MockErrorResponder, r#"{"ok": false, "err": "some_error"}"#);

    #[test]
    fn general_api_error_response() {
        let client = hyper::Client::with_connector(MockErrorResponder::default());
        let result = access_logs(&client, "TEST_TOKEN", None, None);
        assert!(result.is_err());
    }

    mock_slack_responder!(MockAccessLogsOkResponder, r#"
        {
            "ok": true,
            "logins": [
                {
                    "user_id": "U12345",
                    "username": "bob",
                    "date_first": 1422922864,
                    "date_last": 1422922864,
                    "count": 1,
                    "ip": "127.0.0.1",
                    "user_agent": "SlackWeb Mozilla\/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/41.0.2272.35 Safari\/537.36",
                    "isp": "BigCo ISP",
                    "country": "US",
                    "region": "CA"
                },
                {
                    "user_id": "U45678",
                    "username": "alice",
                    "date_first": 1422922493,
                    "date_last": 1422922493,
                    "count": 1,
                    "ip": "127.0.0.1",
                    "user_agent": "SlackWeb Mozilla\/5.0 (iPhone; CPU iPhone OS 8_1_3 like Mac OS X) AppleWebKit\/600.1.4 (KHTML, like Gecko) Version\/8.0 Mobile\/12B466 Safari\/600.1.4",
                    "isp": "BigCo ISP",
                    "country": "US",
                    "region": "CA"
                }
            ],
            "paging": {
                "count": 100,
                "total": 2,
                "page": 1,
                "pages": 1
            }
        }
    "#);

    #[test]
    fn access_logs_ok_response() {
        let client = hyper::Client::with_connector(MockAccessLogsOkResponder::default());
        let result = access_logs(&client, "TEST_TOKEN", None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.logins[0].username, "bob");
        assert_eq!(result.logins[1].username, "alice");
    }

    mock_slack_responder!(MockInfoOkResponder, r#"
        {
            "ok": true,
            "team": {
                "id": "T12345",
                "name": "My Team",
                "domain": "example",
                "email_domain": "",
                "icon": {
                    "image_34": "https:\/\/...",
                    "image_44": "https:\/\/...",
                    "image_68": "https:\/\/...",
                    "image_88": "https:\/\/...",
                    "image_102": "https:\/\/...",
                    "image_132": "https:\/\/...",
                    "image_default": true
                }
            }
        }
    "#);

    #[test]
    fn info_ok_response() {
        let client = hyper::Client::with_connector(MockInfoOkResponder::default());
        let result = info(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.team.name, "My Team");
        assert_eq!(result.team.icon.image_default, true);
    }
}
