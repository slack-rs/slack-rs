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

extern crate hyper;
extern crate websocket;
extern crate rustc_serialize;
pub extern crate slack_api as api;

pub mod error;
pub use error::Error;

pub use api::{Attachment, Channel, Group, Im, Team, User, Message};

mod events;
pub use events::Event;

use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::{self, channel};
use std::thread;

use rustc_serialize::json;

use websocket::Client;
pub use websocket::message::Message as WebSocketMessage;
use websocket::result::WebSocketResult;
use websocket::client::Sender as WsSender;
use websocket::ws::sender::Sender as WsSenderTrait;
use websocket::ws::receiver::Receiver as WsReceiverTrait;
use websocket::client::Receiver as WsReceiver;
use websocket::message::Type as WsType;
use websocket::stream::WebSocketStream;
use websocket::result::WebSocketError;

pub type WsClient = Client<websocket::dataframe::DataFrame,
                           WsSender<websocket::stream::WebSocketStream>,
                           WsReceiver<websocket::stream::WebSocketStream>>;

/// Implement this trait in your code to handle message events
pub trait EventHandler {
    /// When a message is received this will be called with self, the slack client,
    /// and the result of parsing the event received, as well as the raw json string.
    fn on_event(&mut self, cli: &mut RtmClient, event: Result<Event, Error>, raw_json: &str);

    /// Called when a ping is received; you do NOT need to handle the reply pong,
    /// but you may use this event to track the connection as a keep-alive.
    fn on_ping(&mut self, cli: &mut RtmClient);

    /// Called when the connection is closed for any reason.
    fn on_close(&mut self, cli: &mut RtmClient);

    /// Called when the connection is opened.
    fn on_connect(&mut self, cli: &mut RtmClient);
}

/// Used for passing websocket messages in channels
pub enum WsMessage {
    Close,
    Text(String),
    Pong(String),
}

/// The actual messaging client.
pub struct RtmClient {
    token: String,
    start_info: Option<api::rtm::StartResponse>,
    channels: Vec<Channel>,
    groups: Vec<Group>,
    users: Vec<User>,
    channel_ids: HashMap<String, String>,
    group_ids: HashMap<String, String>,
    user_ids: HashMap<String, String>,
    msg_num: Arc<AtomicIsize>,
    outs: Option<mpsc::Sender<WsMessage>>,
}

/// Thread-safe API for sending messages asynchronously
pub struct Sender {
    inner: mpsc::Sender<WsMessage>,
    msg_num: Arc<AtomicIsize>
}

impl Sender {
    /// Get the next message id
    ///
    /// A value returned from this method *must* be included in the JSON payload
    /// (the `id` field) when constructing your own message.
    pub fn get_msg_uid(&self) -> isize {
        self.msg_num.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a raw message
    ///
    /// Must set `message.id` using result of `get_msg_id()`.
    ///
    /// Success from this API does not guarantee the message is delivered
    /// successfully since that runs on a separate task.
    pub fn send(&self, raw: &str) -> Result<(), Error> {
        try!(self.inner.send(WsMessage::Text(raw.to_string()))
                       .map_err(|err| Error::Internal(format!("{}", err))));
        Ok(())
    }

    /// Send a message to the specified channel id
    ///
    /// Success from this API does not guarantee the message is delivered
    /// successfully since that runs on a separate task.
    pub fn send_message_chid(&self, chan_id: &str, msg: &str) -> Result<isize, Error> {
        let n = self.get_msg_uid();
        let msg_json = format!("{}", json::as_json(&msg));
        let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                           n,
                           chan_id,
                           &msg_json[1..msg_json.len() - 1]);

        try!(self.send(&mstr[..]));
        Ok(n)
    }
}

impl RtmClient {

    /// Creates a new client from a token
    pub fn new(token: &str) -> RtmClient {
        RtmClient {
            token: String::from(token),
            start_info: None,
            channels: Vec::new(),
            groups: Vec::new(),
            users: Vec::new(),
            channel_ids: HashMap::new(),
            group_ids: HashMap::new(),
            user_ids: HashMap::new(),
            msg_num: Arc::new(AtomicIsize::new(0)),
            outs: None,
        }
    }

    /// Returns the name of the bot/user connected to the client.
    /// Only valid after login, otherwise None.
    pub fn get_name(&self) -> Option<String> {
        match self.start_info {
            Some(ref s) => Some(s.self_data.name.clone()),
            None => None,
        }
    }

    /// Returns the id of the bot/user connected to the client.
    /// Only valid after login, otherwise None.
    pub fn get_id(&self) -> Option<String> {
        match self.start_info {
            Some(ref s) => Some(s.self_data.id.clone()),
            None => None,
        }
    }

    /// Returns the Team struct of the bot/user connected to the client.
    /// / Only valid after login, otherwise None.
    pub fn get_team(&self) -> Option<Team> {
        match self.start_info {
            Some(ref s) => Some(s.team.clone()),
            None => None,
        }
    }

    /// Get a user id from a username
    /// Only valid after login.
    pub fn get_user_id(&self, username: &str) -> Option<&String> {
        self.user_ids.get(username)
    }

    /// Get a channel id from a channel name, note that channel_name does not begin with a '#'
    /// Only valid after login.
    pub fn get_channel_id(&self, channel_name: &str) -> Option<&String> {
        self.channel_ids.get(channel_name)
    }

    /// Get a group id from a group name
    /// Only valid after login.
    pub fn get_group_id(&self, group_name: &str) -> Option<&String> {
        self.group_ids.get(group_name)
    }

    /// Returns a vector of Users from the team the bot/client is connected to.
    /// Only valid after login.
    pub fn get_users(&self) -> Vec<User> {
        self.users.clone()
    }

    /// Returns a vector of Channels from the team the bot/client is connected to.
    /// Only valid after login.
    pub fn get_channels(&self) -> Vec<Channel> {
        self.channels.clone()
    }

    /// Returns a vector of Groups from the team the bot/client is connected to.
    /// Only valid after login.
    pub fn get_groups(&self) -> Vec<Group> {
        self.groups.clone()
    }

    /// Returns a vector of Ims received on login the bot/client is connected to.
    /// Only valid after login, otherwise None.
    pub fn get_start_ims(&self) -> Option<Vec<Im>> {
        match self.start_info {
            Some(ref s) => Some(s.ims.clone()),
            None => None,
        }
    }


    ///Returns a unique identifier to be used in the 'id' field of a message
    ///sent to slack.
    pub fn get_msg_uid(&self) -> isize {
        self.msg_num.fetch_add(1, Ordering::SeqCst)
    }

    /// Get a thread-safe message sender
    pub fn channel(&self) -> Option<Sender> {
        self.outs.as_ref().cloned().map(|send| Sender {
            inner: send,
            msg_num: self.msg_num.clone(),
        })
    }


    /// Allows sending a json string message over the websocket connection.
    /// Note that this only passes the message over a channel to the
    /// Messaging task, and therfore a succesful return value does not
    /// mean the message has been actually put on the wire yet.
    /// Note that you will need to form a valid json reply yourself if you
    /// use this method, and you will also need to retrieve a unique id for
    /// the message via RtmClient.get_msg_uid()
    /// Only valid after login.
    pub fn send(&mut self, s: &str) -> Result<(), Error> {
        let tx = match self.outs {
            Some(ref tx) => tx,
            None => return Err(Error::Internal(String::from("Failed to get tx!"))),
        };
        try!(tx.send(WsMessage::Text(s.to_string()))
               .map_err(|err| Error::Internal(format!("{}", err))));
        Ok(())
    }

    /// Allows sending a textual string message over the websocket connection,
    /// to the requested channel id. Ideal usage would be EG:
    /// extract the channel in on_receive and then send back a message to the channel.
    /// Note that this only passes the message over a channel to the
    /// Messaging task, and therfore a succesful return value does not
    /// mean the message has been actually put on the wire yet.
    /// This method also handles getting a unique id and formatting the actual json
    /// sent.
    /// Only valid after login.
    pub fn send_message(&self, chan: &str, msg: &str) -> Result<isize, Error> {
        let n = self.get_msg_uid();
        // fixup the channel id if chan is: `#<channel>`
        let chan_id = match chan.starts_with("#") {
            true => {
                match self.get_channel_id(&chan[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Internal(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => chan,
        };
        let msg_json = format!("{}", json::as_json(&msg));
        let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                           n,
                           chan_id,
                           &msg_json[1..msg_json.len() - 1]);
        let tx = match self.outs {
            Some(ref tx) => tx,
            None => return Err(Error::Internal(String::from("Failed to get tx!"))),
        };
        try!(tx.send(WsMessage::Text(mstr))
               .map_err(|err| Error::Internal(format!("{:?}", err))));
        Ok(n)
    }

    /// Logs in to slack. Call this before calling run.
    /// Alternatively use login_and_run
    pub fn login(&mut self) -> Result<(WsClient, mpsc::Receiver<WsMessage>), Error> {
        let client = hyper::Client::new();
        let start = try!(api::rtm::start(&client, &self.token, None, None));

        // websocket url
        let wss_url = try!(hyper::Url::parse(&start.url).map_err(|e| hyper::Error::Uri(e)));

        // update id hashmaps
        for ref channel in start.channels.iter() {
            self.channel_ids.insert(channel.name.clone(), channel.id.clone());
        }
        for ref group in start.groups.iter() {
            self.group_ids.insert(group.name.clone(), group.id.clone());
        }
        for ref user in start.users.iter() {
            self.user_ids.insert(user.name.clone(), user.id.clone());
        }
        // update groups, users, channels:
        self.groups = start.groups.clone();
        self.channels = start.channels.clone();
        self.users = start.users.clone();

        // store rtm.Start data
        self.start_info = Some(start);

        // Do websocket connection request
        let req = try!(websocket::client::Client::connect(wss_url.clone()));

        // Do websocket handshake.
        let res = try!(req.send());

        // Validate handshake
        try!(res.validate());

        // setup channels for passing messages
        let (tx, rx) = channel::<WsMessage>();
        self.outs = Some(tx.clone());
        Ok((res.begin(), rx))
    }

    /// Runs the message receive loop
    pub fn run<T: EventHandler>(&mut self, handler: &mut T, client: WsClient, rx: mpsc::Receiver<WsMessage>) -> Result<(), Error> {
        // for sending messages
        let tx = match self.outs {
            Some(ref mut tx) => tx.clone(),
            None => return Err(Error::Internal(String::from("No tx!"))),
        };

        let (mut sender, mut receiver) = client.split();

        handler.on_connect(self);
        // websocket send loop
        // We used thread::scoped previously but it is no longer stable...
        let child = thread::spawn(move || -> () {
            loop {
                let msg = match rx.recv() {
                    Ok(m) => m,
                    Err(_) => {
                        // if we had an error receiving, shutdown the sender
                        // and receiver so that we return.
                        match sender.shutdown_all() {
                            Ok(_) => {},
                            Err(err) => panic!(err),
                        };
                        return;
                    }
                };

                match msg {
                    WsMessage::Close => {
                        drop(rx);
                        return;
                    },
                    WsMessage::Text(text) => {
                        let message = WebSocketMessage::text(text);
                        match sender.send_message(&message) {
                            Ok(_) => {},
                            Err(_) => {
                                // if we had an error sending, shutdown the sender
                                // and receiver so that we return.
                                match sender.shutdown_all() {
                                    Ok(_) => {},
                                    Err(err) => panic!(err),
                                };
                                return;
                            }
                        }
                    },
                    WsMessage::Pong(data) => {
                        let message = WebSocketMessage::pong(data.as_bytes());
                        match sender.send_message(&message) {
                            Ok(_) => {},
                            Err(_) => {
                                // if we had an error sending, shutdown the sender
                                // and receiver so that we return.
                                match sender.shutdown_all() {
                                    Ok(_) => {},
                                    Err(err) => panic!(err),
                                };
                                return;
                            }
                        }
                    }
                };
            }
        });

        // set receive timeout long enough for slack ping
        {
            let read_timeout = std::time::Duration::from_secs(70);
            let mut ws_stream = receiver.get_mut().get_mut();
            let tcp_stream: &mut std::net::TcpStream = match ws_stream {
                &mut WebSocketStream::Tcp(ref mut s) => s,
                &mut WebSocketStream::Ssl(ref mut s) => s.get_mut(),
            };
            try!(tcp_stream.set_read_timeout(Some(read_timeout)));
        }

        // receive loop
        loop {
            // receive
            let message_result : WebSocketResult<WebSocketMessage> = receiver.recv_message();
            // unwrap result
            let message : WebSocketMessage = match message_result {
                Ok(message) => message,
                Err(err) => {
                    // If error is equivalent of EAGAIN, just loop
                    if let WebSocketError::IoError(ref io_err) = err {
                        if io_err.kind() == io::ErrorKind::WouldBlock {
                            continue
                        }
                    }

                    // shutdown sender and receiver, then join the child thread
                    // and return an error.
                    let _ = tx.send(WsMessage::Close);
                    let _ = receiver.shutdown_all();
                    let _ = child.join();
                    return Err(Error::Internal(format!("{:?}", err)));
                }
            };
            // handle the message
            match message.opcode {
                WsType::Text => {
                    let raw_string : String = try!(String::from_utf8(message.payload.into_owned()));
                    match json::decode(&raw_string) {
                        Ok(event) => handler.on_event(self, Ok(event), &raw_string),
                        Err(err) => handler.on_event(self, Err(Error::JsonDecode(err)), &raw_string),
                    }
                }
                WsType::Ping => {
                    handler.on_ping(self);
                    let raw_string : String = try!(String::from_utf8(message.payload.into_owned()));
                    match tx.send(WsMessage::Pong(raw_string)) {
                        Ok(_) => {}
                        Err(err) => {
                            // shutdown sender and receiver, then join the child thread
                            // and return an error.
                            let _ = receiver.shutdown_all();
                            let _ = child.join();
                            return Err(Error::Internal(format!("{:?}", err)));
                        }
                    }
                }
                WsType::Close => {
                    handler.on_close(self);
                    match tx.send(WsMessage::Close) {
                        Ok(_) => {}
                        Err(err) => {
                            // shutdown sender and receiver, then join the child thread
                            // and return an error.
                            let _ = receiver.shutdown_all();
                            let _ = child.join();
                            return Err(Error::Internal(format!("{:?}", err)));
                        }
                    }
                    // close the sender and receiver
                    let _ = receiver.shutdown_all();
                    // join the child thread, return error if the child thread paniced
                    return match child.join() {
                        Ok(_) => Ok(()),
                        Err(err) => Err(Error::Internal(format!("child thread error in run: {:?}", err)))
                    };
                }
                _ => {}
            }
        }
    }

    /// Runs the main loop for the client after logging in to slack,
    /// returns an error if the process fails at an point, or an Ok(()) on succesful
    /// close.
    /// Takes a EventHandler (implemented by the user) to call events handlers on.
    /// once the first on_receive() or on_ping is called on the EventHandler, you
    /// can soon the 'Only valid after login' methods are safe to use.
    /// Sending is run in a thread in parallel while the receive loop runs on the main thread.
    /// Both loops should end on return.
    /// Sending should be thread safe as the messages are passed in via a channel in
    /// RtmClient.send and RtmClient.send_message
    pub fn login_and_run<T: EventHandler>(&mut self, handler: &mut T) -> Result<(), Error> {
        let (client, rx) = try!(self.login());
        self.run(handler, client, rx)
    }

    /// Uses https://api.slack.com/methods/users.list to get a list of users
    pub fn list_users(&mut self) -> Result<Vec<User>, Error> {
        let client = hyper::Client::new();
        let data = try!(api::users::list(&client, &self.token, None));

        Ok(data.members)
    }

    /// Uses https://api.slack.com/methods/channels.list to get a list of channels
    pub fn list_channels(&mut self) -> Result<Vec<Channel>, Error> {
        let client = hyper::Client::new();
        let data = try!(api::channels::list(&client, &self.token, None));

        Ok(data.channels)
    }

    /// Uses https://api.slack.com/methods/groups.list to get a list of groups
    pub fn list_groups(&mut self) -> Result<Vec<Group>, Error> {
        let client = hyper::Client::new();
        let data = try!(api::groups::list(&client, &self.token, None));

        Ok(data.groups)
    }

    /// Uses https://api.slack.com/methods/users.list to update users
    pub fn update_users(&mut self) -> Result<Vec<User>, Error> {
        let users = try!(self.list_users());

        // update user id map
        self.user_ids.clear();
        for ref user in users.iter() {
            self.user_ids.insert(user.name.clone(), user.id.clone());
        }
        // update users
        self.users = users.clone();

        Ok(users)
    }

    /// Uses https://api.slack.com/methods/channels.list to update channels
    pub fn update_channels(&mut self) -> Result<Vec<Channel>, Error> {
        let channels = try!(self.list_channels());

        // update channel id map
        self.channel_ids.clear();
        for ref channel in channels.iter() {
            self.channel_ids.insert(channel.name.clone(), channel.id.clone());
        }
        // update users
        self.channels = channels.clone();

        Ok(channels)
    }

    /// Uses https://api.slack.com/methods/groups.list to update groups
    pub fn update_groups(&mut self) -> Result<Vec<Group>, Error> {
        let groups = try!(self.list_groups());
        // update group id map
        self.group_ids.clear();
        for ref group in groups.iter() {
            self.group_ids.insert(group.name.clone(), group.id.clone());
        }
        // update users
        self.groups = groups.clone();
        Ok(groups)
    }

    /// Wraps https://api.slack.com/methods/chat.postMessage
    /// json_payload can be a json formatted action or simple text that will be posted as a message.
    /// See https://api.slack.com/docs/formatting
    pub fn post_message(&self, channel: &str, json_payload: &str, attachments: Option<&str>) -> Result<api::chat::PostMessageResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        let client = hyper::Client::new();
        api::chat::post_message(&client,
                                &self.token,
                                chan_id,
                                json_payload,
                                None,
                                Some(true),
                                None,
                                None,
                                attachments,
                                None,
                                None,
                                None,
                                None).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/chat.delete to delete a message
    /// See the slack api docs for timestamp formatting.
    pub fn delete_message(&self, channel: &str, timestamp: &str) -> Result<api::chat::DeleteResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        let client = hyper::Client::new();
        api::chat::delete(&client, &self.token, chan_id, timestamp).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.mark to set the read cursor in a channel
    /// See the slack api docs for timestamp formatting.
    pub fn mark(&self, channel: &str, timestamp: &str) -> Result<api::channels::MarkResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        let client = hyper::Client::new();
        api::channels::mark(&client, &self.token, chan_id, timestamp).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.setTopic
    /// if channel starts with a # then it will be looked up with get_channel_id
    /// topic will be json escaped.
    pub fn set_topic(&self, channel: &str, topic: &str) -> Result<api::channels::SetTopicResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        // this will json format the string, which should escape it,
        // we'll need to slice out the quotes around it afterwards
        let escaped_topic = format!("{}", json::as_json(&topic));
        let client = hyper::Client::new();
        api::channels::set_topic(&client,
                                 &self.token,
                                 chan_id,
                                 &escaped_topic[1..escaped_topic.len() - 1]).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.setPurpose
    /// if channel starts with a # then it will be looked up with get_channel_id
    /// purpose will be json escaped.
    pub fn set_purpose(&self, channel: &str, purpose: &str) -> Result<api::channels::SetPurposeResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        // this will json format the string, which should escape it,
        // we'll need to slice out the quotes around it afterwards
        let escaped_purpose = format!("{}", json::as_json(&purpose));
        let client = hyper::Client::new();
        api::channels::set_purpose(&client,
                                   &self.token,
                                   chan_id,
                                   &escaped_purpose[1..escaped_purpose.len() - 1]).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add an emoji reaction to a message
    /// if channel starts with a # then it will be looked up with get_channel_id
    pub fn add_reaction_timestamp(&self, emoji_name: &str, channel: &str, timestamp: &str) -> Result<api::reactions::AddResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        let client = hyper::Client::new();
        api::reactions::add(&client,
                            &self.token,
                            emoji_name,
                            None,
                            None,
                            Some(chan_id),
                            Some(timestamp)).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add an emoji reaction to a file
    pub fn add_reaction_file(&self, emoji_name: &str, file: &str) -> Result<api::reactions::AddResponse, Error> {
        let client = hyper::Client::new();
        api::reactions::add(&client,
                            &self.token,
                            emoji_name,
                            Some(file),
                            None,
                            None,
                            None).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add an emoji reaction to a file comment
    pub fn add_reaction_file_comment(&self, emoji_name: &str, file_comment: &str) -> Result<api::reactions::AddResponse, Error> {
        let client = hyper::Client::new();
        api::reactions::add(&client,
                            &self.token,
                            emoji_name,
                            None,
                            Some(file_comment),
                            None,
                            None).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/chat.update
    /// json_payload can be a json formatted action or simple text that will be posted as a message.
    /// See https://api.slack.com/docs/formatting
    pub fn update_message(&self, channel: &str, timestamp: &str, json_payload: &str, attachments: Option<&str>) -> Result<api::chat::UpdateResponse, Error> {
        // fixup the channel id if channel is: `#<channel>`
        let chan_id = match channel.starts_with("#") {
            true => {
                match self.get_channel_id(&channel[1..]) {
                    Some(s) => &(s[..]),
                    None => return Err(Error::Api(String::from("start_info is invalid, need to login first"))),
                }
            }
            false => channel,
        };
        let client = hyper::Client::new();
        api::chat::update(&client,
                          &self.token,
                          timestamp,
                          chan_id,
                          json_payload,
                          attachments,
                          None,
                          None).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.open to open a direct message channel with a user.
    pub fn im_open(&self, user_id: &str) -> Result<api::im::OpenResponse, Error> {
        let client = hyper::Client::new();
        api::im::open(&client, &self.token, user_id).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.history to retrieve the history of messages and
    /// events from a channel.
    pub fn channels_history(&self,
                            channel_id: &str,
                            latest: Option<&str>,
                            oldest: Option<&str>,
                            inclusive: Option<bool>,
                            count: Option<u32>)
                            -> Result<api::channels::HistoryResponse, Error> {
        let client = hyper::Client::new();
        api::channels::history(&client,
                               &self.token,
                               channel_id,
                               latest,
                               oldest,
                               inclusive,
                               count).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.close to close a direct message channel.
    pub fn im_close(&self, channel_id: &str) -> Result<api::im::CloseResponse, Error> {
        let client = hyper::Client::new();
        api::im::close(&client, &self.token, channel_id).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.history to retrieve the history of messages and
    /// events from a direct message channel.
    pub fn im_history(&self,
                      channel_id: &str,
                      latest: Option<&str>,
                      oldest: Option<&str>,
                      inclusive: Option<bool>,
                      count: Option<u32>)
                      -> Result<api::im::HistoryResponse, Error> {
        let client = hyper::Client::new();
        api::im::history(&client,
                         &self.token,
                         channel_id,
                         latest,
                         oldest,
                         inclusive,
                         count).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.list to get the list of all open direct message
    /// channels the user has open.
    pub fn im_list(&self) -> Result<api::im::ListResponse, Error> {
        let client = hyper::Client::new();
        api::im::list(&client, &self.token).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.mark to move the read cursor in a direct message
    /// channel.
    pub fn im_mark(&self, channel_id: &str, timestamp: &str) -> Result<api::im::MarkResponse, Error> {
        let client = hyper::Client::new();
        api::im::mark(&client, &self.token, channel_id, timestamp).map_err(|e| e.into())
    }
}
