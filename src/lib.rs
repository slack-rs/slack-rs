/*
Copyright 2014 Benjamin Elder from https://github.com/BenTheElder/slack-rs

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! #Notes:
//! - Structs except for RtmClient are derived from the slack api docs at: https://api.slack.com/
//!    - Optional fields in Slack structs represent fields that may not be present in the serialized form.
//!    - Currently fields in Slack structs are public, but in the future they may be wrapped in getters.
//!
//! - Usage: Implement an EventHandler to handle slack events and messages in conjunction with RtmClient.
//!
//! #Changelog:
//! Version 0.9.1 -- With help from https://github.com/mthjones, overhaul error handling and refactor.
//!
//!  - Introduced slack::Error
//!
//!  - Exposed list_users, list_groups, list_channels
//!
//! Version 0.8.3 -- Moved example to examples dir thanks to https://github.com/mthjones: https://github.com/BenTheElder/slack-rs/pull/9
//!
//! Version 0.8.2 -- Fix https://github.com/BenTheElder/slack-rs/issues/8
//!
//! Version 0.8.1 -- Add some web api methods, add methods to map names to ids.
//!
//!  - TODO: expect the error type overhaul to be pushed back to 0.9.X
//!
//!  - NOTE: Compatibility changes from 0.7.X include: RtmClient::new now takes the bot token/api_key and login,
//! login_and_run do not.
//!
//!
//! Version 0.7.2 -- Bugfix via https://github.com/Farthen: https://github.com/BenTheElder/slack-rs/pull/6
//!
//! Version 0.7.1 -- Cleaned up the api and json handling.
//!
//! - TODO: expect better error handling in 0.8.X
//!
//! - NOTE: Compatibility changes from 0.6.1 include: 'MessageHandler' is now 'EventHandler' and all of the
//! slack data structs have been updated to match the api as closely as possible.
//!
//! Version 0.6.1 -- Updated to stable rust.

extern crate hyper;
extern crate websocket;
extern crate openssl;
extern crate rustc_serialize;
extern crate url;

use std::sync::mpsc::{Sender,Receiver,channel};
use std::thread;
use std::io::Read;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::collections::HashMap;

use rustc_serialize::json;

use websocket::Client;
pub use websocket::message::Message;
use websocket::Sender as WsSender;
use websocket::Receiver as WsReceiver;
use websocket::dataframe::DataFrame;
use websocket::stream::WebSocketStream;

pub type WsClient = Client<websocket::dataframe::DataFrame,
                           websocket::client::sender::Sender<websocket::stream::WebSocketStream>,
                           websocket::client::receiver::Receiver<websocket::stream::WebSocketStream>>;

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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Internal(format!("{:?}", err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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



/// Implement this trait in your code to handle message events
pub trait EventHandler {
	/// When a message is received this will be called with self, the slack client,
	/// and the json encoded string payload.
	fn on_receive(&mut self, cli: &mut RtmClient, json_str: &str);

	/// Called when a ping is received; you do NOT need to handle the reply pong,
	/// but you may use this event to track the connection as a keep-alive.
	fn on_ping(&mut self, cli: &mut RtmClient);

	/// Called when the connection is closed for any reason.
	fn on_close(&mut self, cli: &mut RtmClient);

	/// Called when the connection is opened.
	fn on_connect(&mut self, cli: &mut RtmClient);
}

/// See the slack api docs at: https://api.slack.com/
// Currently missing "latest" field
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub created: i64,
    pub creator: String,
    pub is_archived: bool,
    pub is_general: bool,
    pub members: Option<Vec<String>>,
    pub topic: Option<Topic>,
    pub purpose: Option<Purpose>,
    pub is_member: bool,
    pub last_read: Option<String>,
    pub unread_count: Option<i64>,
    pub unread_count_display: Option<i64>,
}

/// See the slack api docs at: https://api.slack.com/
// Currently missing "latest" field
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Group {
	pub id: String,
	pub name: String,
	pub is_group: bool,
	pub created: i64,
	pub creator:  String,
	pub is_archived:  bool,
	pub members: Option<Vec<String>>,
	pub topic: Option<Topic>,
	pub purpose: Option<Purpose>,
	pub last_read: Option<String>,
	pub unread_count: Option<i64>,
	pub unread_count_display: Option<i64>,
}


/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub deleted: bool,
    pub is_bot: bool,
    pub tz_offset: Option<i64>,
}

/// See the slack api docs at: https://api.slack.com/
// We've left out the prefs field for now
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct SelfData {
    pub id: String,
    pub name: String,
    pub created: i64,
    pub manual_presence: String,
}

/// See the slack api docs at: https://api.slack.com/
// We've left out the prefs field for now
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Team {
	pub id: String,
    pub name: String,
    pub email_domain: String,
	pub domain: String,
    pub msg_edit_window_mins: i64,
	pub over_storage_limit: bool,
    pub plan: String,
}

/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Topic {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Purpose {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Im {
	pub id: String,
	pub is_im: bool,
	pub user:  String,
	pub created: i64,
	pub is_user_deleted: Option<bool>,
}

/// See the slack api docs at: https://api.slack.com/
// Bots field ignored for now
//#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(dead_code)]
pub struct RtmStart {
  pub ok: bool,
	pub url: String,
	pub self_data: SelfData,
	pub team: Team,
	pub users: Vec<User>,
	pub channels: Vec<Channel>,
	pub groups: Vec<Group>,
	pub ims: Vec<Im>,
}

// This is an ugly hack, we have to compile with --pretty expanded and fix up self to map to self_data.
// An alternative would be using serde, but it won't do what we need on stable.
impl ::rustc_serialize::Decodable for RtmStart {
    fn decode<__D: ::rustc_serialize::Decoder>(__arg_0: &mut __D)
     -> ::std::result::Result<RtmStart, __D::Error> {
        __arg_0.read_struct("RtmStart", 8usize, |_d| -> _ {
                            ::std::result::Result::Ok(RtmStart{ok:
                                                                   match _d.read_struct_field("ok",
                                                                                              0usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               url:
                                                                   match _d.read_struct_field("url",
                                                                                              1usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               self_data:
                                                                   match _d.read_struct_field("self",
                                                                                              2usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               team:
                                                                   match _d.read_struct_field("team",
                                                                                              3usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               users:
                                                                   match _d.read_struct_field("users",
                                                                                              4usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               channels:
                                                                   match _d.read_struct_field("channels",
                                                                                              5usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               groups:
                                                                   match _d.read_struct_field("groups",
                                                                                              6usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },
                                                               ims:
                                                                   match _d.read_struct_field("ims",
                                                                                              7usize,
                                                                                              ::rustc_serialize::Decodable::decode)
                                                                       {
                                                                       ::std::result::Result::Ok(__try_var)
                                                                       =>
                                                                       __try_var,
                                                                       ::std::result::Result::Err(__try_var)
                                                                       =>
                                                                       return ::std::result::Result::Err(__try_var),
                                                                   },}) })
    }
}

// used only by update_users
#[allow(dead_code)]
#[derive(RustcDecodable)]
struct UserListResponse {
    ok: bool,
    err: Option<String>,
    members: Option<Vec<User>>,
}

// used only by update_groups
#[allow(dead_code)]
#[derive(RustcDecodable)]
struct GroupListResponse {
    ok: bool,
    err: Option<String>,
    groups: Option<Vec<Group>>,
}

// used only by update_channels
#[allow(dead_code)]
#[derive(RustcDecodable)]
struct ChannelListResponse {
    ok: bool,
    err: Option<String>,
    channels: Option<Vec<Channel>>,
}

/// The actual messaging client.
pub struct RtmClient {
    token: String,
    start_info: Option<RtmStart>,
    channels: Vec<Channel>,
    groups: Vec<Group>,
    users: Vec<User>,
    channel_ids: HashMap<String, String>,
    group_ids: HashMap<String, String>,
    user_ids: HashMap<String, String>,
	msg_num: AtomicIsize,
	outs : Option<Sender<Message>>
}


impl RtmClient {

	/// Creates a new client from a token
	pub fn new(token: &str) -> RtmClient {
		RtmClient{
            token: String::from(token),
            start_info : None,
            channels: Vec::new(),
            groups: Vec::new(),
            users: Vec::new(),
            channel_ids: HashMap::new(),
            group_ids: HashMap::new(),
            user_ids: HashMap::new(),
			msg_num: AtomicIsize::new(0),
			outs : None
		}
	}


    /// Returns the sending half of the channel used internally for sending messages.
    /// Prefer send_message or send to this.
    /// Only valid after login, otherwise None.
	pub fn get_message_sender(&self) -> Option<Sender<Message>> {
		self.outs.clone()
	}

	/// Returns the name of the bot/user connected to the client.
	/// Only valid after login, otherwise None.
	pub fn get_name(&self) -> Option<String> {
        match self.start_info {
		    Some(ref s) => Some(s.self_data.name.clone()),
            None => None
        }
	}

	/// Returns the id of the bot/user connected to the client.
	/// Only valid after login, otherwise None.
	pub fn get_id(&self) -> Option<String> {
        match self.start_info {
		    Some(ref s) => Some(s.self_data.id.clone()),
            None => None
        }
	}

	/// Returns the Team struct of the bot/user connected to the client.
	//// Only valid after login, otherwise None.
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
	pub fn get_start_ims(&self) ->  Option<Vec<Im>> {
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


	/// Allows sending a json string message over the websocket connection.
	/// Note that this only passes the message over a channel to the
	/// Messaging task, and therfore a succesful return value does not
	/// mean the message has been actually put on the wire yet.
	/// Note that you will need to form a valid json reply yourself if you
	/// use this method, and you will also need to retrieve a unique id for
	/// the message via RtmClient.get_msg_uid()
	/// Only valid after login.
	pub fn send(&mut self, s : &str) -> Result<(), Error> {
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err(Error::Internal(String::from("Failed to get tx!")))
		};
	    try!(tx.send(Message::Text(s.to_string())).map_err(|err| Error::Internal(format!("{}", err))));
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
	pub fn send_message(&self, chan: &str, msg: &str) -> Result<(), Error>{
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
		let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,n,chan_id, &msg_json[1..msg_json.len()-1]);
        println!("{}", mstr);
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err(Error::Internal(String::from("Failed to get tx!")))
		};
		try!(tx.send(Message::Text(mstr)).map_err(|err| Error::Internal(format!("{:?}", err))));
		Ok(())
	}

	/// Logs in to slack. Call this before calling run.
	/// Alternatively use login_and_run
	pub fn login(&mut self) -> Result<(WsClient, Receiver<Message>), Error> {
		let mut res = try!(self.make_authed_api_call("rtm.start", HashMap::new()));

		// Read result string
		let mut res_str = String::new();
		try!(res.read_to_string(&mut res_str));

        // Parse json
        let start: RtmStart = try!(json::decode(&res_str));

        // check "ok" field
        if !start.ok {
            return Err(Error::Api(format!("slack api error: (ok not true)")));
        }

        // websocket url
		let wss_url = try!(hyper::Url::parse(&start.url));

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
		let (tx,rx) = channel::<Message>();
		self.outs = Some(tx.clone());
		Ok((res.begin(), rx))
	}

	/// Runs the message receive loop
	pub fn run<T: EventHandler>(&mut self, handler: &mut T, client: WsClient, rx: Receiver<Message>) -> Result<(), Error> {
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
					Ok(m) => { m },
					Err(_) => { return; }
				};

				let closing = match msg {
					Message::Close(_) => { true },
					_ => { false }
				};
				match sender.send_message(msg) {
					Ok(_) => {},
					Err(_) => { return; }
				}
				if closing {
					drop(rx);
					return;
				}
			}
		});

		// receive loop
		for message in receiver.incoming_messages() {
			let message = match message {
				Ok(message) => message,
				Err(err) => {
					let _ = child.join();
					return Err(Error::Internal(format!("{:?}", err)));
				}
			};

			match message {
				Message::Text(data) => {
					handler.on_receive(self, &data);
				},
				Message::Ping(data) => {
					handler.on_ping(self);
					let message = Message::Pong(data);
					match tx.send(message) {
						Ok(_) => {},
						Err(err) => {
							let _ = child.join();
							return Err(Error::Internal(format!("{:?}", err)));
						}
					}
				},
				Message::Close(data) => {
					handler.on_close(self);
					let message = Message::Close(data);
					match tx.send(message) {
						Ok(_) => {},
						Err(err) => {
							let _ = child.join();
							return Err(Error::Internal(format!("{:?}", err)));
						}
					}
					let _ = child.join();
					return Ok(());
				},
				_ => {}
			}
		}
		let _ = child.join();
		Ok(())
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
        let mut res = try!(self.make_authed_api_call("users.list", HashMap::new()));

        // Read result string
        let mut res_str = String::new();
        try!(res.read_to_string(&mut res_str));

        // now that we know it isn't an error, decode
        let data: UserListResponse = try!(json::decode(&res_str));

        if let Some(err) = data.err {
            return Err(Error::Api(format!("Got a slack error: {}", err)));
        }

        let members = match data.members {
            Some(m) => m,
            None => return Err(Error::Api(String::from("Members field missing in users.List response!"))),
        };

        Ok(members)
    }

    /// Uses https://api.slack.com/methods/channels.list to get a list of channels
    pub fn list_channels(&mut self) -> Result<Vec<Channel>, Error> {
        let mut res = try!(self.make_authed_api_call("channels.list", HashMap::new()));

        // Read result string
        let mut res_str = String::new();
        try!(res.read_to_string(&mut res_str));

        // now that we know it isn't an error, decode
        let data: ChannelListResponse = try!(json::decode(&res_str));

        if let Some(err) = data.err {
            return Err(Error::Api(format!("Got a slack error: {}", err)));
        }

        let channels = match data.channels {
            Some(c) => c,
            None => return Err(Error::Api(String::from("Channels field missing in users.List response!"))),
        };

        Ok(channels)
    }

    /// Uses https://api.slack.com/methods/groups.list to get a list of groups
    pub fn list_groups(&mut self) -> Result<Vec<Group>, Error> {
        let mut res = try!(self.make_authed_api_call("groups.list", HashMap::new()));

        // Read result string
        let mut res_str = String::new();
        try!(res.read_to_string(&mut res_str));

        // now that we know it isn't an error, decode
        let data: GroupListResponse = try!(json::decode(&res_str));

        if let Some(err) = data.err {
            return Err(Error::Api(format!("Got a slack error: {}", err)));
        }

        let groups = match data.groups {
            Some(c) => c,
            None => return Err(Error::Api(String::from("Channels field missing in users.List response!"))),
        };

        Ok(groups)
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
    pub fn post_message(&self, channel: &str, json_payload: &str, attachments: Option<String>) -> Result<hyper::client::response::Response, Error> {
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
        let mut params = HashMap::new();
        params.insert("channel", chan_id);
        params.insert("text", json_payload);
        params.insert("as_user", "true");
        if let Some(ref a) = attachments {
            params.insert("attachments", a);
        }
        self.make_authed_api_call("chat.postMessage", params)
    }

    /// Wraps https://api.slack.com/methods/channels.setTopic
    /// if channel starts with a # then it will be looked up with get_channel_id
    /// topic will be json escaped.
    pub fn set_topic(&self, channel: &str, topic: &str) -> Result<hyper::client::response::Response, Error> {
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
        let escaped_topic = format!("{}",json::as_json(&topic));
        let mut params = HashMap::new();
        params.insert("channel", chan_id);
        params.insert("topic", &escaped_topic[..]);
        self.make_authed_api_call("channels.setTopic", params)
    }

    /// Wraps https://api.slack.com/methods/channels.setPurpose
    /// if channel starts with a # then it will be looked up with get_channel_id
    /// purpose will be json escaped.
    pub fn set_purpose(&self, channel: &str, purpose: &str) -> Result<hyper::client::response::Response, Error> {
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
        let escaped_purpose = format!("{}",json::as_json(&purpose));
        let mut params = HashMap::new();
        params.insert("channel", chan_id);
        params.insert("purpose", &escaped_purpose[..]);
        self.make_authed_api_call("channels.setTopic", params)
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add an emoji reaction to a message
    /// if channel starts with a # then it will be looked up with get_channel_id
    pub fn add_reaction_timestamp(&self, emoji_name: &str, channel: &str, timestamp: &str) -> Result<hyper::client::response::Response, Error> {
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
        let mut params = HashMap::new();
        params.insert("name", emoji_name);
        params.insert("channel", chan_id);
        params.insert("timestamp", timestamp);
        self.make_authed_api_call("reactions.add", params)
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add an emoji reaction to a file
    pub fn add_reaction_file(&self, emoji_name: &str, file: &str) -> Result<hyper::client::response::Response, Error> {
        let mut params = HashMap::new();
        params.insert("name", emoji_name);
        params.insert("file", file);
        self.make_authed_api_call("reactions.add", params)
    }

    /// Wraps https://api.slack.com/methods/reactions.add to add an emoji reaction to a file comment
    pub fn add_reaction_file_comment(&self, emoji_name: &str, file_comment: &str) -> Result<hyper::client::response::Response, Error> {
        let mut params = HashMap::new();
        params.insert("name", emoji_name);
        params.insert("file_comment", file_comment);
        self.make_authed_api_call("reactions.add", params)
    }

    /// Make an API call to Slack that includes the configured token. Takes a map of parameters
    /// that get appended to the request as query params.
    fn make_authed_api_call<'a>(&'a self, method: &str, mut custom_params: HashMap<&str, &'a str>) -> Result<hyper::client::response::Response, Error> {
        let url_string = format!("https://slack.com/api/{}", method);
        let mut url = try!(hyper::Url::parse(&url_string));

        custom_params.insert("token", &self.token[..]);
        url.set_query_from_pairs(custom_params.into_iter());

        Ok(try!(hyper::Client::new().get(url).send()))
    }
}
