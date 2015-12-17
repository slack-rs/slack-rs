use std::fmt;
use std::io;
use std::error;
use std::string::FromUtf8Error;

use hyper;
use websocket;
use url;
use rustc_serialize;

/// slack::Error represents errors that can happen while using the RtmClient
#[derive(Debug)]
pub enum Error {
    /// Http client error
    Http(hyper::Error),
    /// WebSocket connection error
    WebSocket(websocket::result::WebSocketError),
    /// Error decoding websocket text frame Utf8
    Utf8(FromUtf8Error),
    /// Error parsing url
    Url(url::ParseError),
    /// Error decoding Json
    JsonDecode(rustc_serialize::json::DecoderError),
    /// Error parsing Json
    JsonParse(rustc_serialize::json::ParserError),
    /// Error encoding Json
    JsonEncode(rustc_serialize::json::EncoderError),
    /// Slack Api Error
    Api(String),
    /// Errors that do not fit under the other types, Internal is for EG channel errors.
    Internal(String),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<websocket::result::WebSocketError> for Error {
    fn from(err: websocket::result::WebSocketError) -> Error {
        Error::WebSocket(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::Url(err)
    }
}

impl From<rustc_serialize::json::DecoderError> for Error {
    fn from(err: rustc_serialize::json::DecoderError) -> Error {
        Error::JsonDecode(err)
    }
}

impl From<rustc_serialize::json::ParserError> for Error {
    fn from(err: rustc_serialize::json::ParserError) -> Error {
        Error::JsonParse(err)
    }
}

impl From<rustc_serialize::json::EncoderError> for Error {
    fn from(err: rustc_serialize::json::EncoderError) -> Error {
        Error::JsonEncode(err)
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http (hyper) Error: {:?}", e),
            Error::WebSocket(ref e) => write!(f, "Websocket Error: {:?}", e),
            Error::Utf8(ref e) => write!(f, "Utf8 decode Error: {:?}", e),
            Error::Url(ref e) => write!(f, "Url Error: {:?}", e),
            Error::JsonDecode(ref e) => write!(f, "Json Decode Error: {:?}", e),
            Error::JsonParse(ref e) => write!(f, "Json Parse Error: {:?}", e),
            Error::JsonEncode(ref e) => write!(f, "Json Encode Error: {:?}", e),
            Error::Api(ref st) => write!(f, "Slack Api Error: {:?}", st),
            Error::Internal(ref st) => write!(f, "Internal Error: {:?}", st)
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
            Error::JsonDecode(ref e) => e.description(),
            Error::JsonParse(ref e) => e.description(),
            Error::JsonEncode(ref e) => e.description(),
            Error::Api(ref st) => st,
            Error::Internal(ref st) => st
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::WebSocket(ref e) => Some(e),
            Error::Utf8(ref e) => Some(e),
            Error::Url(ref e) => Some(e),
            Error::JsonDecode(ref e) => Some(e),
            Error::JsonParse(ref e) => Some(e),
            Error::JsonEncode(ref e) => Some(e),
            Error::Api(_) => None,
            Error::Internal(_) => None
        }
    }
}
