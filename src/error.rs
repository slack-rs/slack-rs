use std::fmt;
use std::io;

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
    /// Error parsing url
    Url(url::ParseError),
    /// Error decoding Json
    JsonDecode(rustc_serialize::json::DecoderError),
    /// Error encoding Json
    JsonEncode(rustc_serialize::json::EncoderError),
    /// Slack Api Error
    Api(String),
    /// Errors that do not fit under the other types, Internal is for EG channel errors.
    Internal(String)
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Error::Http(ref e) => format!("Http (hyper) Error: {:?}", e),
            Error::WebSocket(ref e) => format!("Http (hyper) Error: {:?}", e),
            Error::Url(ref e) => format!("Url Error: {:?}", e),
            Error::JsonDecode(ref e) => format!("Json Decode Error: {:?}", e),
            Error::JsonEncode(ref e) => format!("Json Encode Error: {:?}", e),
            Error::Api(ref st) => format!("Slack Api Error: {:?}", st),
            Error::Internal(ref st) => format!("Internal Error: {:?}", st)
        };
        f.write_str(&s)
    }
}
