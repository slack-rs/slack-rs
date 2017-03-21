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
extern crate native_tls;

pub mod error;
pub use error::Error;

pub use api::{Channel, Group, Im, Team, User, Message};

mod events;
pub use events::Event;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::{self, channel};
use native_tls::TlsStream;
use std::net::TcpStream;

pub type SlackWebsocket = tungstenite::WebSocket<tungstenite::stream::Stream<TcpStream,
                                                                             TlsStream<TcpStream>>>;
/// Implement this trait in your code to handle message events
pub trait EventHandler {
    /// When a message is received this will be called with self, the slack client,
    /// and the result of parsing the event received, as well as the raw json string.
    fn on_event(&mut self, cli: &mut RtmClient, event: Result<Event, Error>, raw_json: &str);

    /// Called when the connection is closed for any reason.
    fn on_close(&mut self, cli: &mut RtmClient);

    /// Called when the connection is opened.
    fn on_connect(&mut self, cli: &mut RtmClient);
}

/// Used for passing websocket messages in channels
pub enum WsMessage {
    Close,
    Text(String),
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
    msg_num: Arc<AtomicIsize>,
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
        try!(self.inner
            .send(WsMessage::Text(raw.to_string()))
            .map_err(|err| Error::Internal(format!("{}", err))));
        Ok(())
    }

    /// Send a message to the specified channel id
    ///
    /// Success from this API does not guarantee the message is delivered
    /// successfully since that runs on a separate task.
    pub fn send_message_chid(&self, chan_id: &str, msg: &str) -> Result<isize, Error> {
        let n = self.get_msg_uid();
        let msg_json = serde_json::to_string(&msg)?;
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

    // TODO: remove these? they are not returned in:
    // https://docs.rs/slack_api/0.16.0/slack_api/rtm/struct.StartResponse.html

    // /// Returns the name of the bot/user connected to the client.
    // /// Only valid after login, otherwise None.
    // pub fn get_name(&self) -> Option<String> {
    //     match self.start_info {
    //         Some(ref s) => Some(s.self_data.name.clone()),
    //         None => None,
    //     }
    // }
    //
    // /// Returns the id of the bot/user connected to the client.
    // /// Only valid after login, otherwise None.
    // pub fn get_id(&self) -> Option<String> {
    //     match self.start_info {
    //         Some(ref s) => Some(s.self_data.id.clone()),
    //         None => None,
    //     }
    // }
    //
    /// Returns the Team struct of the bot/user connected to the client.
    /// / Only valid after login, otherwise None.
    pub fn get_team(&self) -> Option<Team> {
        match self.start_info {
            Some(ref s) => s.team.clone(),
            None => None,
        }
    }

    /// Get a user id from a username
    /// Only valid after login.
    pub fn get_user_id(&self, username: &str) -> Option<&String> {
        self.user_ids.get(username)
    }

    /// Evaluate if chan is a channel name or channel id
    /// If channel name, returns its id
    /// If channel id, returns itself
    /// Only valid after login.
    fn evaluate_channel_id(&self, chan: &str) -> Result<String, Error> {
        let id = if chan.starts_with('#') {
            match self.get_channel_id(&chan[1..]) {
                Some(s) => s,
                None => return Err(Error::Internal(String::from("need to login first to retrieve channel list"))),
            }
        } else {
            chan
        };

        Ok(id.to_string())
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
            Some(ref s) => s.ims.clone(),
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
        self.outs
            .as_ref()
            .cloned()
            .map(|send| {
                     Sender {
                         inner: send,
                         msg_num: self.msg_num.clone(),
                     }
                 })
    }

    /// Allows sending a json string message over the websocket connection.
    /// Note that this only passes the message over a channel to the
    /// Messaging task, and therefore a successful return value does not
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
        try!(tx.send(WsMessage::Text(s.to_string())).map_err(|err| {
                                                                 Error::Internal(format!("{}", err))
                                                             }));
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

        let chan_id = match self.evaluate_channel_id(chan) {
            Ok(id) => id,
            _ => return Err(Error::Internal(String::from("Failed to get channel id"))),
        };

        let msg_json = serde_json::to_string(&msg)?;
        let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                           n,
                           chan_id,
                           &msg_json[1..msg_json.len() - 1]);
        let tx = match self.outs {
            Some(ref tx) => tx,
            None => return Err(Error::Internal(String::from("Failed to get tx!"))),
        };
        try!(tx.send(WsMessage::Text(mstr)).map_err(|err| Error::Internal(format!("{:?}", err))));
        Ok(n)
    }

    /// Marks connected client as being typing to a channel
    /// This is mostly used to signal to other peers that a message
    /// is being typed. Will have the server send a "user_typing" message to all the
    /// peers.
    /// Slack doc can be found at https://api.slack.com/rtm under "Typing Indicators"
    pub fn send_typing(&self, chan: &str) -> Result<isize, Error> {
        let n = self.get_msg_uid();

        let chan_id = match self.evaluate_channel_id(chan) {
            Ok(id) => id,
            _ => return Err(Error::Internal(String::from("Failed to get channel id"))),
        };

        let mstr = format!(r#"{{"id": {}, "type": "typing", "channel": "{}"}}"#,
                           n,
                           chan_id);

        let tx = match self.outs {
            Some(ref tx) => tx,
            None => return Err(Error::Internal(String::from("Failed to get tx!"))),
        };

        try!(tx.send(WsMessage::Text(mstr)).map_err(|err| Error::Internal(format!("{:?}", err))));
        Ok(n)
    }

    /// Logs in to slack. Call this before calling run.
    /// Alternatively use login_and_run
    pub fn login(&mut self) -> Result<(SlackWebsocket, mpsc::Receiver<WsMessage>), Error> {
        let client = reqwest::Client::new()?;
        let start = try!(api::rtm::start(&client, &self.token, &Default::default()));
        let start_url = &start.url.clone().expect("websocket url from slack");

        // websocket url
        let wss_url = reqwest::Url::parse(start_url)?;

        // update id hashmaps
        if let Some(ref channels) = start.channels {
            for channel in channels {
                self.channel_ids.insert(channel.name.clone().unwrap(), channel.id.clone().unwrap());
            }
            self.channels = channels.clone();
        }
        if let Some(ref groups) = start.groups {
            for group in groups {
                self.group_ids.insert(group.name.clone().unwrap(), group.id.clone().unwrap());
            }
            self.groups = groups.clone();
        }

        if let Some(ref users) = start.users {
            for user in users {
                self.user_ids.insert(user.name.clone().unwrap(), user.id.clone().unwrap());
            }
            self.users = users.clone();
        }

        // store rtm.Start data
        self.start_info = Some(start);

        let ws = tungstenite::connect(wss_url)?;

        // setup channels for passing messages
        let (tx, rx) = channel::<WsMessage>();
        self.outs = Some(tx.clone());
        Ok((ws, rx))
    }

    /// Runs the message receive loop
    pub fn run<T: EventHandler>(&mut self,
                                handler: &mut T,
                                mut websocket: SlackWebsocket,
                                rx: mpsc::Receiver<WsMessage>)
                                -> Result<(), Error> {
        handler.on_connect(self);
        // receive loop
        loop {
            // try to write out pending messages (if any)
            loop {
                match rx.try_recv() {
                    Ok(msg) => {
                        match msg {
                            WsMessage::Text(text) => {
                                websocket.write_message(tungstenite::Message::Text(text))?
                            }
                            WsMessage::Close => {
                                handler.on_close(self);
                                return websocket.close().map_err(|e| e.into());
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
                    let event = serde_json::from_str::<Event>(&text[..]);
                    match event {
                        Ok(event) => handler.on_event(self, Ok(event), &text),
                        Err(err) => handler.on_event(self, Err(Error::Json(err)), &text),
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
    pub fn login_and_run<T: EventHandler>(&mut self, handler: &mut T) -> Result<(), Error> {
        let (client, rx) = try!(self.login());
        self.run(handler, client, rx)
    }


    /// Shutdown `RtmClient`
    pub fn shutdown(&self) -> Result<(), Error> {
        match self.outs {
            Some(ref tx) => {
                tx.send(WsMessage::Close)
                    .map_err(|_| Error::Internal("Error sending shutdown message".into()))
            }
            None => Err(Error::Internal("Cannot shutdown without a sender".into())),
        }
    }

    /// Uses https://api.slack.com/methods/users.list to get a list of users
    pub fn list_users(&mut self) -> Result<Option<Vec<User>>, Error> {
        let client = reqwest::Client::new()?;
        let data = try!(api::users::list(&client, &self.token, &Default::default()));

        Ok(data.members)
    }

    /// Uses https://api.slack.com/methods/channels.list to get a list of channels
    pub fn list_channels(&mut self) -> Result<Option<Vec<Channel>>, Error> {
        let client = reqwest::Client::new()?;
        let data = try!(api::channels::list(&client, &self.token, &Default::default()));

        Ok(data.channels)
    }

    /// Uses https://api.slack.com/methods/groups.list to get a list of groups
    pub fn list_groups(&mut self) -> Result<Option<Vec<Group>>, Error> {
        let client = reqwest::Client::new()?;
        let data = try!(api::groups::list(&client, &self.token, &Default::default()));

        Ok(data.groups)
    }

    /// Uses https://api.slack.com/methods/users.list to update users
    pub fn update_users(&mut self) -> Result<Vec<User>, Error> {
        let users = try!(self.list_users());

        match users {
            Some(users) => {
                // update user id map
                self.user_ids.clear();
                for user in &users {
                    self.user_ids.insert(user.name.clone().unwrap(), user.id.clone().unwrap());
                }
                // update users
                self.users = users.clone();

                Ok(users)
            }
            None => Err(Error::Api("No users found".into())),
        }
    }

    /// Uses https://api.slack.com/methods/channels.list to update channels
    pub fn update_channels(&mut self) -> Result<Vec<Channel>, Error> {
        let channels = try!(self.list_channels());

        match channels {
            Some(channels) => {
                // update channel id map
                self.channel_ids.clear();
                for channel in &channels {
                    self.channel_ids.insert(channel.name.clone().unwrap(),
                                            channel.id.clone().unwrap());
                }
                // update users
                self.channels = channels.clone();

                Ok(channels)

            }
            None => Err(Error::Api("No channels found".into())),
        }

    }

    /// Uses https://api.slack.com/methods/groups.list to update groups
    pub fn update_groups(&mut self) -> Result<Vec<Group>, Error> {
        let groups = try!(self.list_groups());
        match groups {
            Some(groups) => {
                // update group id map
                self.group_ids.clear();
                for group in &groups {
                    self.group_ids.insert(group.name.clone().unwrap(), group.id.clone().unwrap());
                }
                // update users
                self.groups = groups.clone();
                Ok(groups)

            }
            None => Err(Error::Api("No groups found".into())),
        }
    }

    /// Wraps https://api.slack.com/methods/chat.postMessage
    /// json_payload can be a json formatted action or simple text that will be posted as a message.
    /// See https://api.slack.com/docs/formatting
    pub fn post_message(&self,
                        message: &api::chat::PostMessageRequest)
                        -> Result<api::chat::PostMessageResponse, Error> {
        let client = reqwest::Client::new()?;
        api::chat::post_message(&client, &self.token, message).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/chat.delete to delete a message
    /// See the slack api docs for timestamp formatting.
    pub fn delete_message(&self,
                          req: &api::chat::DeleteRequest)
                          -> Result<api::chat::DeleteResponse, Error> {
        let client = reqwest::Client::new()?;
        api::chat::delete(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.mark to set the read cursor in a channel
    /// See the slack api docs for timestamp formatting.
    pub fn mark(&self,
                req: &api::channels::MarkRequest)
                -> Result<api::channels::MarkResponse, Error> {
        let client = reqwest::Client::new()?;
        api::channels::mark(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.setTopic
    /// if channel starts with a # then it will be looked up with get_channel_id
    /// topic will be json escaped.
    pub fn set_topic(&self,
                     req: &api::channels::SetTopicRequest)
                     -> Result<api::channels::SetTopicResponse, Error> {
        let client = reqwest::Client::new()?;
        api::channels::set_topic(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.setPurpose
    /// if channel starts with a # then it will be looked up with get_channel_id
    /// purpose will be json escaped.
    pub fn set_purpose(&self,
                       req: &api::channels::SetPurposeRequest)
                       -> Result<api::channels::SetPurposeResponse, Error> {
        let client = reqwest::Client::new()?;
        api::channels::set_purpose(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add a reaction to a message
    pub fn add_reaction(&self,
                        req: &api::reactions::AddRequest)
                        -> Result<api::reactions::AddResponse, Error> {
        let client = reqwest::Client::new()?;
        api::reactions::add(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/chat.update
    /// json_payload can be a json formatted action or simple text that will be posted as a message.
    /// See https://api.slack.com/docs/formatting
    pub fn update_message(&self,
                          req: &api::chat::UpdateRequest)
                          -> Result<api::chat::UpdateResponse, Error> {
        let client = reqwest::Client::new()?;
        api::chat::update(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.open to open a direct message channel with a user.
    pub fn im_open(&self, req: &api::im::OpenRequest) -> Result<api::im::OpenResponse, Error> {
        let client = reqwest::Client::new()?;
        api::im::open(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/channels.history to retrieve the history of messages and
    /// events from a channel.
    pub fn channels_history(&self,
                            req: &api::channels::HistoryRequest)
                            -> Result<api::channels::HistoryResponse, Error> {
        let client = reqwest::Client::new()?;
        api::channels::history(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.close to close a direct message channel.
    pub fn im_close(&self, req: &api::im::CloseRequest) -> Result<api::im::CloseResponse, Error> {
        let client = reqwest::Client::new()?;
        api::im::close(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.history to retrieve the history of messages and
    /// events from a direct message channel.
    pub fn im_history(&self,
                      req: &api::im::HistoryRequest)
                      -> Result<api::im::HistoryResponse, Error> {
        let client = reqwest::Client::new()?;
        api::im::history(&client, &self.token, req).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.list to get the list of all open direct message
    /// channels the user has open.
    pub fn im_list(&self) -> Result<api::im::ListResponse, Error> {
        let client = reqwest::Client::new()?;
        api::im::list(&client, &self.token).map_err(|e| e.into())
    }

    /// Wraps https://api.slack.com/methods/im.mark to move the read cursor in a direct message
    /// channel.
    pub fn im_mark(&self, req: &api::im::MarkRequest) -> Result<api::im::MarkResponse, Error> {
        let client = reqwest::Client::new()?;
        api::im::mark(&client, &self.token, req).map_err(|e| e.into())
    }
}
