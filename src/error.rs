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

use crate::api;
use std::error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

/// `slack::Error` represents errors that can happen while using the `RtmClient`
#[derive(Debug)]
pub enum Error {
    /// Http client error
    Http(api::requests::Error),
    /// WebSocket connection error
    WebSocket(::tungstenite::Error),
    /// Error decoding websocket text frame Utf8
    Utf8(FromUtf8Error),
    /// Error parsing url
    Url(::url::ParseError),
    /// Error decoding Json
    Json(::serde_json::Error),
    /// Slack Api Error
    Api(String),
    /// Errors that do not fit under the other types, Internal is for EG channel errors.
    Internal(String),
}

impl From<api::requests::Error> for Error {
    fn from(err: api::requests::Error) -> Error {
        Error::Http(err)
    }
}

impl From<::url::ParseError> for Error {
    fn from(err: ::url::ParseError) -> Error {
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

impl From<api::rtm::StartError<api::requests::Error>> for Error {
    fn from(err: api::rtm::StartError<api::requests::Error>) -> Error {
        Error::Api(format!("rtm::StartError: {}", err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http (reqwest) Error: {}", e),
            Error::WebSocket(ref e) => write!(f, "Websocket Error: {}", e),
            Error::Utf8(ref e) => write!(f, "Utf8 decode Error: {}", e),
            Error::Url(ref e) => write!(f, "Url Error: {}", e),
            Error::Json(ref e) => write!(f, "Json Error: {}", e),
            Error::Api(ref st) => write!(f, "Slack Api Error: {}", st),
            Error::Internal(ref st) => write!(f, "Internal Error: {}", st),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::WebSocket(ref e) => Some(e),
            Error::Utf8(ref e) => Some(e),
            Error::Url(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::Api(_) | Error::Internal(_) => None,
        }
    }
}
