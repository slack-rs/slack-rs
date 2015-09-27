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
//!    - Currently fields in Slack structs are public, but in 0.8.X onward they will likely be wrapped in getters.
//!
//! - Usage: Implement an EventHandler to handle slack events and messages in conjunction with RtmClient.
//!
//! #Changelog:
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

use rustc_serialize::json;
use std::sync::mpsc::{Sender,Receiver,channel};
use std::thread;
use std::io::Read;
use std::sync::atomic::{AtomicIsize, Ordering};
use websocket::Client;
pub use websocket::message::Message;
use websocket::Sender as WsSender;
use websocket::Receiver as WsReceiver;
use websocket::dataframe::DataFrame;
use websocket::stream::WebSocketStream;
use websocket::client::request::Url;

pub type WsClient = Client<websocket::dataframe::DataFrame,
                           websocket::client::sender::Sender<websocket::stream::WebSocketStream>,
                           websocket::client::receiver::Receiver<websocket::stream::WebSocketStream>>;



///Implement this trait in your code to handle message events
pub trait EventHandler {
	///When a message is received this will be called with self, the slack client,
	///and the json encoded string payload.
	fn on_receive(&mut self, cli: &mut RtmClient, json_str: &str);

	///Called when a ping is received; you do NOT need to handle the reply pong,
	///but you may use this event to track the connection as a keep-alive.
	fn on_ping(&mut self, cli: &mut RtmClient);

	///Called when the connection is closed for any reason.
	fn on_close(&mut self, cli: &mut RtmClient);

	///Called when the connection is opened.
	fn on_connect(&mut self, cli: &mut RtmClient);
}

/// See the slack api docs at: https://api.slack.com/
// Currently missing "latest" field
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub created: i64,
    pub creator: String,
    pub is_archived: bool,
    pub is_general: bool,
    pub members: Vec<String>,
    pub topic: Topic,
    pub purpose: Purpose,
    pub is_member: bool,
    pub last_read: String,
    pub unread_count: i64,
    pub unread_count_display: i64,
}

/// See the slack api docs at: https://api.slack.com/
// Currently missing "latest" field
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Group {
	pub id: String,
	pub name: String,
	pub is_group: bool,
	pub created: i64,
	pub creator:  String,
	pub is_archived:  bool,
	pub members: Vec<String>,
	pub topic: Topic,
	pub purpose: Purpose,
	pub last_read: String,
	pub unread_count: i64,
	pub unread_count_display: i64,
}


/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone)]
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
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SelfData {
    pub id: String,
    pub name: String,
    pub created: i64,
    pub manual_presence: String,
}

/// See the slack api docs at: https://api.slack.com/
// We've left out the prefs field for now
#[derive(RustcDecodable, RustcEncodable, Clone)]
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
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Topic {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Purpose {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

/// See the slack api docs at: https://api.slack.com/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Im {
	pub id: String,
	pub is_im: bool,
	pub user:  String,
	pub created: i64,
	pub is_user_deleted: Option<bool>,
}

/// See the slack api docs at: https://api.slack.com/
// Bots field ignored for now
//#[derive(RustcDecodable, RustcEncodable)]
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



///The actual messaging client.
pub struct RtmClient {
    start_info: Option<RtmStart>,
	msg_num: AtomicIsize,
	outs : Option<Sender<Message>>
}

///Error string. (FIXME: better error return values/ custom error type)
static RTM_INVALID : &'static str = "Invalid data returned from slack (rtm.start)";


impl RtmClient {

	/// Creates a new empty client.
	pub fn new() -> RtmClient {
		RtmClient{
            start_info : None,
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

    /// Returns a vector of Users from the team the bot/client is connected to.
	/// Only valid after login, otherwise None.
    pub fn get_users(&self) -> Option<Vec<User>> {
        match self.start_info {
            Some(ref s) => Some(s.users.clone()),
            None => None,
        }
    }

    /// Returns a vector of Channels from the team the bot/client is connected to.
	/// Only valid after login, otherwise None.
	pub fn get_channels(&self) -> Option<Vec<Channel>> {
        match self.start_info {
            Some(ref s) => Some(s.channels.clone()),
            None => None,
        }
    }

    /// Returns a vector of Groups from the team the bot/client is connected to.
	/// Only valid after login, otherwise None.
	pub fn get_groups(&self) -> Option<Vec<Group>> {
        match self.start_info {
            Some(ref s) => Some(s.groups.clone()),
            None => None,
        }
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
	pub fn send(&mut self, s : &str) -> Result<(),String> {
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err("Failed to get tx!".to_string())
		};
		match tx.send(Message::Text(s.to_string())) {
			Ok(_) => {},
			Err(err) => return Err(format!("{:?}", err))
		}
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
	pub fn send_message(&self, chan: &str, msg: &str) -> Result<(), String>{
		let n = self.get_msg_uid();
        // fixup the channel id if chan is: `#<channel>`
        let chan_id: String = match chan.starts_with("#") {
            true => {
                match self.start_info {
                    Some(ref s) => {
                        let mut id = String::new();
                        for channel in &s.channels {
                            if channel.name == chan[1..] {
                                id = channel.id.clone();
                                break
                            }
                        }
                        id
                    },
                    None => return Err(String::from("start_info is invalid, need to login first")),
                }
            }
            false => String::from(chan),
        };
		let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,n,chan_id,msg);
        println!("{}", mstr);
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err("Failed to get tx!".to_string())
		};
		match tx.send(Message::Text(mstr)) {
			Ok(_) => {},
			Err(err) => return Err(format!("{:?}", err))
		}
		Ok(())
	}

	/// Logs in to slack. Call this before calling run.
	/// Alternatively use login_and_run
	pub fn login(&mut self, token: &str) -> Result<(WsClient, Receiver<Message>), String> {
		//Slack real time api url
		let url = "https://slack.com/api/rtm.start?token=".to_string() + token;

		// Create http client and send request to slack
		let client = hyper::Client::new();
		let mut res = match client.get(&url).send() {
			Ok(res) => res,
			Err(err) => return Err(format!("Hyper Error: {:?}", err))
		};

		// Read result string
		let mut res_str = String::new();
		match res.read_to_string(&mut res_str) {
			Err(err) => return Err(format!("{:?}", err)),
			_ => {},
		};


        // Parse json
        let start: RtmStart = match json::decode(&res_str) {
            Ok(s) => s,
            Err(err) => return Err(format!("{:?}", err)),
        };

        // check "ok" field
        if !start.ok {
            return Err(format!("{} (ok not true)", RTM_INVALID));
        }

        // websocket url
		let wss_url = match Url::parse(&start.url) {
			Ok(url) => url,
			Err(err) => return Err(format!("{:?}", err))
		};

        // store rtm.Start data
        self.start_info = Some(start);

        // Do websocket connection request
		let req = match websocket::client::Client::connect(wss_url.clone()) {
			Ok(res) => res,
			Err(err) => return Err(format!("{:?}, Websocket request to `{:?}` failed", err, wss_url))
		};

		// Do websocket handshake.
		let res = match req.send() {
			Ok(res) => res,
			Err(err) => {
				return Err(format!("{:?}, Websocket request to `{:?}` failed", err, wss_url))
			}
		};

        // Validate handshake
		match res.validate() {
			Ok(()) => { }
			Err(err) => {
				return Err(format!("Error: res.validate(): {:?}", err))
			}
		}

        // setup channels for passing messages
		let (tx,rx) = channel::<Message>();
		self.outs = Some(tx.clone());
		Ok((res.begin(),rx))
	}

	/// Runs the message receive loop
	pub fn run<T: EventHandler>(&mut self, handler: &mut T, client: WsClient, rx: Receiver<Message>) -> Result<(),String> {
		// for sending messages
		let tx = match self.outs {
			Some(ref mut tx) => { tx.clone() },
			None => { return Err("No tx!".to_string()); }
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
					return Err(format!("{:?}", err));
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
							return Err(format!("{:?}", err));
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
							return Err(format!("{:?}", err));
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
	pub fn login_and_run<T: EventHandler>(&mut self, handler: &mut T, token : &str) -> Result<(),String> {
		let (client,rx) = match self.login(token) {
			Ok((c,r)) => { (c,r) },
			Err(err) => { return Err(format!("{:?}",err)); }
		};
		self.run(handler, client, rx)
	}
}
