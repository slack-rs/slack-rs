//
// Copyright 2014-2016 the slack-rs authors.
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

//! Slack realtime messaging client: https://api.slack.com/bot-users
//!
//! See [CHANGELOG.md](https://github.com/slack-rs/slack-rs/blob/master/CHANGELOG.md) for latest
//! release notes.

extern crate reqwest;
pub extern crate slack_api as api;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tungstenite;

pub mod error;
pub use error::Error;

pub use api::{Channel, Group, Im, Team, User, Message};

mod events;
pub use events::Event;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, channel};
use events::{MessageSent, MessageError};

/// Implement this trait in your code to handle message events
pub trait EventHandler {
    /// When a message is received this will be called with self, the slack client,
    /// and the result of parsing the event received, as well as the raw json string.
    fn on_event(&mut self, cli: &RtmClient, event: Result<Event, Error>);

    /// Called when the connection is closed for any reason.
    fn on_close(&mut self, cli: &RtmClient);

    /// Called when the connection is opened.
    fn on_connect(&mut self, cli: &RtmClient);
}

/// Used for passing websocket messages in channels
#[derive(Debug)]
pub enum WsMessage {
    Close,
    Text(String),
}

/// The actual messaging client.
pub struct RtmClient {
    start_response: api::rtm::StartResponse,
    sender: Sender,
    rx: mpsc::Receiver<WsMessage>,
}

/// Thread-safe API for sending messages asynchronously
#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<WsMessage>,
    msg_num: Arc<AtomicUsize>,
}

impl Sender {
    /// Get the next message id
    ///
    /// A value returned from this method *must* be included in the JSON payload
    /// (the `id` field) when constructing your own message.
    pub fn get_msg_uid(&self) -> usize {
        self.msg_num.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a raw message
    ///
    /// Must set `message.id` using result of `get_msg_id()`.
    ///
    /// Success from this API does not guarantee the message is delivered
    /// successfully since that runs on a separate task.
    pub fn send(&self, raw: &str) -> Result<(), Error> {
        self.tx
            .send(WsMessage::Text(raw.to_string()))
            .map_err(|err| Error::Internal(format!("{}", err)))?;
        Ok(())
    }

    /// Send a message to the specified channel id
    ///
    /// Success from this API does not guarantee the message is delivered
    /// successfully since that runs on a separate task.
    pub fn send_message_chid(&self, chan_id: &str, msg: &str) -> Result<usize, Error> {
        let n = self.get_msg_uid();
        let msg_json = serde_json::to_string(&msg)?;
        let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                           n,
                           chan_id,
                           &msg_json[1..msg_json.len() - 1]);

        self.send(&mstr[..])?;
        Ok(n)
    }
}

impl RtmClient {
    /// Logs in to slack. Call this before calling run.
    /// Alternatively use login_and_run
    pub fn login(token: &str) -> Result<RtmClient, Error> {
        let client = reqwest::Client::new()?;
        let start_response = api::rtm::start(&client, token, &Default::default())?;

        // setup channels for passing messages
        let (tx, rx) = channel::<WsMessage>();
        let sender = Sender {
            tx: tx,
            msg_num: Arc::new(AtomicUsize::new(0)),
        };

        Ok(RtmClient {
               start_response: start_response,
               sender: sender,
               rx: rx,
           })
    }

    /// Runs the message receive loop
    pub fn run<T: EventHandler>(&self, handler: &mut T) -> Result<(), Error> {
        let start_url = match self.start_response.url {
            Some(ref url) => url,
            None => return Err(Error::Api("Slack did not provide a URL".into())),
        };

        let wss_url = reqwest::Url::parse(&start_url)?;
        let mut websocket = tungstenite::connect(wss_url)?;

        handler.on_connect(self);
        // receive loop
        loop {
            // try to write out pending messages (if any)
            loop {
                match self.rx.try_recv() {
                    Ok(msg) => {
                        match msg {
                            WsMessage::Text(text) => {
                                websocket
                                    .write_message(tungstenite::Message::Text(text))?
                            }
                            WsMessage::Close => {
                                handler.on_close(self);
                                return websocket.close(None).map_err(|e| e.into());
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        handler.on_close(self);
                        return Err(Error::Internal("rx disconnected".into()));
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                }
            }

            // blocks until a message is received or websocket errors
            let message = websocket.read_message()?;

            // handle the message
            match message {
                tungstenite::Message::Text(text) => {
                    match Event::from_json(&text[..]) {
                        Ok(event) => handler.on_event(self, Ok(event)),
                        Err(err) => {
                            println!("raw = {}", text);
                            handler.on_event(self, Err(err))
                        }
                    }
                }
                tungstenite::Message::Binary(_) => {}
            }
        }
    }

    /// Runs the main loop for the client after logging in to slack,
    /// returns an error if the process fails at any point, or an Ok(()) on successful
    /// close.
    /// Takes an EventHandler (implemented by the user) to call events handlers on.
    /// Once the first on_receive() or on_ping is called on the EventHandler, you
    /// can assume the 'Only valid after login' methods are safe to use.
    /// Sending is run in a thread in parallel while the receive loop runs on the main thread.
    /// Both loops should end on return.
    /// Sending should be thread safe as the messages are passed in via a channel in
    /// RtmClient.send and RtmClient.send_message
    pub fn login_and_run<T: EventHandler>(token: &str, handler: &mut T) -> Result<(), Error> {
        let client = RtmClient::login(token)?;
        client.run(handler)
    }

    /// Shutdown `RtmClient`
    pub fn shutdown(&self) -> Result<(), Error> {
        self.sender
            .tx
            .send(WsMessage::Close)
            .map_err(|_| Error::Internal("Error sending shutdown message".into()))
    }

    ///Returns a unique identifier to be used in the 'id' field of a message
    ///sent to slack.
    pub fn get_msg_uid(&self) -> usize {
        self.sender.msg_num.fetch_add(1, Ordering::SeqCst)
    }

    /// Get a thread-safe message sender
    pub fn sender(&self) -> Sender {
        self.sender.clone()
    }

    /// Allows sending a json string message over the websocket connection.
    /// Note that this only passes the message over a channel to the
    /// Messaging task, and therefore a successful return value does not
    /// mean the message has been actually put on the wire yet.
    /// Note that you will need to form a valid json reply yourself if you
    /// use this method, and you will also need to retrieve a unique id for
    /// the message via RtmClient.get_msg_uid()
    /// Only valid after login.
    pub fn send(&self, s: &str) -> Result<(), Error> {
        self.sender
            .tx
            .send(WsMessage::Text(s.to_string()))
            .map_err(|err| Error::Internal(format!("{}", err)))
    }

    /// Allows sending a textual string message over the websocket connection,
    /// to the requested channel id. Ideal usage would be EG:
    /// extract the channel in on_receive and then send back a message to the channel.
    /// Note that this only passes the message over a channel to the
    /// Messaging task, and therefore a successful return value does not
    /// mean the message has been actually put on the wire yet.
    /// This method also handles getting a unique id and formatting the actual json
    /// sent.
    /// Only valid after login.
    ///
    /// `channel_id` is the slack channel id, e.g. `UXYZ1234`, not `#general`.
    pub fn send_message(&self, channel_id: &str, msg: &str) -> Result<usize, Error> {
        let n = self.get_msg_uid();
        let msg_json = serde_json::to_string(&msg)?;
        let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                           n,
                           channel_id,
                           &msg_json[1..msg_json.len() - 1]);
        self.sender
            .tx
            .send(WsMessage::Text(mstr))
            .map_err(|err| Error::Internal(format!("{:?}", err)))?;
        Ok(n)
    }

    /// Marks connected client as being typing to a channel
    /// This is mostly used to signal to other peers that a message
    /// is being typed. Will have the server send a "user_typing" message to all the
    /// peers.
    /// Slack doc can be found at https://api.slack.com/rtm under "Typing Indicators"
    ///
    /// `channel_id` is the slack channel id, e.g. `UXYZ1234`, not `#general`.
    pub fn send_typing(&self, channel_id: &str) -> Result<usize, Error> {
        let n = self.get_msg_uid();
        let mstr = format!(r#"{{"id": {}, "type": "typing", "channel": "{}"}}"#,
                           n,
                           channel_id);

        self.sender
            .tx
            .send(WsMessage::Text(mstr))
            .map_err(|err| Error::Internal(format!("{:?}", err)))?;
        Ok(n)
    }

    /// Returns a reference to the `StartResponse`.
    pub fn start_response(&self) -> &api::rtm::StartResponse {
        &self.start_response
    }
}

impl Event {
    /// Try to deserialize an `Event` from a json-encoded `&str`
    fn from_json(s: &str) -> Result<Event, Error> {
        match serde_json::from_str::<Event>(s) {
            Ok(ev) => Ok(ev),
            Err(e) => {
                // try for the MessageSent / MessageError variants that don't expose type
                if let Ok(ev) = serde_json::from_str::<MessageSent>(s) {
                    Ok(Event::MessageSent(ev))
                } else if let Ok(ev) = serde_json::from_str::<MessageError>(s) {
                    Ok(Event::MessageError(ev))
                } else {
                    Err(e.into())
                }
            }
        }
    }
}
