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

use std::fmt;
use std::io;
use std::error;
use std::string::FromUtf8Error;

use api;

/// slack::Error represents errors that can happen while using the RtmClient
#[derive(Debug)]
pub enum Error {
    /// Http client error
    Http(::reqwest::Error),
    /// WebSocket connection error
    WebSocket(::tungstenite::Error),
    /// Error decoding websocket text frame Utf8
    Utf8(FromUtf8Error),
    /// Error parsing url
    Url(::reqwest::UrlError),
    /// Error decoding Json
    Json(::serde_json::Error),
    /// Slack Api Error
    Api(String),
    /// Errors that do not fit under the other types, Internal is for EG channel errors.
    Internal(String),
}

impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<::reqwest::UrlError> for Error {
    fn from(err: ::reqwest::UrlError) -> Error {
        Error::Url(err)
    }
}

impl From<::tungstenite::Error> for Error {
    fn from(err: ::tungstenite::Error) -> Error {
        Error::WebSocket(err)
    }
}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Internal(format!("{:?}", err))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}

/// helper macro to make `impl From<>` for api errors.
macro_rules! impl_api_from {
    {
        $name:ty
    } => {
        impl From<$name> for Error {
            fn from(err: $name) -> Error {
                Error::Api(format!("{}: {:?}", stringify!($name), err))
            }
        }
    }
}

impl_api_from!(api::rtm::StartError<::reqwest::Error>);
impl_api_from!(api::users::ListError<::reqwest::Error>);
impl_api_from!(api::channels::ListError<::reqwest::Error>);
impl_api_from!(api::channels::MarkError<::reqwest::Error>);
impl_api_from!(api::channels::SetTopicError<::reqwest::Error>);
impl_api_from!(api::channels::SetPurposeError<::reqwest::Error>);
impl_api_from!(api::channels::HistoryError<::reqwest::Error>);
impl_api_from!(api::reactions::AddError<::reqwest::Error>);
impl_api_from!(api::groups::ListError<::reqwest::Error>);
impl_api_from!(api::chat::PostMessageError<::reqwest::Error>);
impl_api_from!(api::chat::UpdateError<::reqwest::Error>);
impl_api_from!(api::chat::DeleteError<::reqwest::Error>);
impl_api_from!(api::im::ListError<::reqwest::Error>);
impl_api_from!(api::im::OpenError<::reqwest::Error>);
impl_api_from!(api::im::CloseError<::reqwest::Error>);
impl_api_from!(api::im::HistoryError<::reqwest::Error>);
impl_api_from!(api::im::MarkError<::reqwest::Error>);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http (reqwest) Error: {:?}", e),
            Error::WebSocket(ref e) => write!(f, "Websocket Error: {:?}", e),
            Error::Utf8(ref e) => write!(f, "Utf8 decode Error: {:?}", e),
            Error::Url(ref e) => write!(f, "Url Error: {:?}", e),
            Error::Json(ref e) => write!(f, "Json Error: {:?}", e),
            Error::Api(ref st) => write!(f, "Slack Api Error: {:?}", st),
            Error::Internal(ref st) => write!(f, "Internal Error: {:?}", st),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::WebSocket(ref e) => e.description(),
            Error::Utf8(ref e) => e.description(),
            Error::Url(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::Api(ref st) => st,
            Error::Internal(ref st) => st,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::WebSocket(ref e) => Some(e),
            Error::Utf8(ref e) => Some(e),
            Error::Url(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::Api(_) => None,
            Error::Internal(_) => None,
        }
    }
}
