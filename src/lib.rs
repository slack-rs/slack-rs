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
extern crate hyper;
extern crate websocket;
extern crate "rustc-serialize" as rustc_serialize;
extern crate url;

use rustc_serialize::json::{Json};
use std::sync::mpsc::{Sender,channel};
use std::thread::Thread;
use std::sync::atomic::{AtomicIsize, Ordering};
use websocket::message::WebSocketMessage;
use websocket::handshake::WebSocketRequest;
use url::Url;


///Implement this trait in your code to handle message events
pub trait MessageHandler {
	///When a message is received this will be called with self, the slack client,
	///and the json encoded string payload.
	fn on_receive(&mut self, cli: &mut RtmClient, json_str: &str);

	///Called when a ping is received; you do NOT need to handle the reply pong,
	///but you may use this event to track the connection as a keep-alive.
	fn on_ping(&mut self, cli: &mut RtmClient);

	///Called when the connection is closed for any reason.
	fn on_close(&mut self, cli: &mut RtmClient);
}


///Contains information about the team the bot is logged into.
pub struct Team {
	name : String,
	id : String
}

impl Team {
	///private, create empty team.
	fn new() -> Team {
		Team{name: String::new(), id: String::new()}
	}

	///Returns the team's name as a String
	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	///Returns the team's id as a String
	pub fn get_id(&self) -> String {
		self.id.clone()
	}
}

///The actual messaging client.
pub struct RtmClient {
	name : String,
	id : String,
	team : Team,
	msg_num: AtomicIsize,
	outs : Option<Sender<String>>
}

///Error string. (FIXME: better error return values/ custom error type)
static RTM_INVALID : &'static str = "Invalid data returned from slack (rtm.start)";


impl RtmClient {

	///Creates a new empty client.
	pub fn new() -> RtmClient {
		RtmClient{
			name : String::new(),
			id : String::new(),
			team : Team::new(),
			msg_num: AtomicIsize::new(0),
			outs : None
		}
	}

	///Returns the name of the bot/user connected to the client.
	///Only valid after login.
	pub fn get_name(&self) -> String {
		return self.name.clone();
	}

	///Returns the id of the bot/user connected to the client.
	///Only valid after login.
	pub fn get_id(&self) -> String {
		return self.id.clone();
	}

	///Returns the Team struct of the bot/user connected to the client.
	///Only valid after login.
	pub fn get_team<'a>(&'a self) -> &'a Team {
		&self.team
	}

	///Returns a unique identifier to be used in the 'id' field of a message
	///sent to slack.
	pub fn get_msg_uid(&self) -> isize {
		self.msg_num.fetch_add(1, Ordering::SeqCst)
	}


	///Allows sending a json string message over the websocket connection.
	///Note that this only passes the message over a channel to the
	///Messaging task, and therfore a succesful return value does not
	///mean the message has been actually put on the wire yet.
	///Note that you will need to form a valid json reply yourself if you
	///use this method, and you will also need to retrieve a unique id for
	///the message via RtmClient.get_msg_uid()
	///Only valid after login.
	pub fn send(&mut self, s : &str) -> Result<(),String> {
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err("Failed to get tx!".to_string())
		};
		match tx.send(s.to_string()) {
			Ok(_) => {},
			Err(err) => return Err(format!("{:?}", err))
		}
		Ok(())
	}

	///Allows sending a textual string message over the websocket connection,
	///to the requested channel id. Ideal usage would be EG:
	///extract the channel in on_receive and then send back a message to the channel.
	///Note that this only passes the message over a channel to the
	///Messaging task, and therfore a succesful return value does not
	///mean the message has been actually put on the wire yet.
	///This method also handles getting a unique id and formatting the actual json
	///sent.
	///Only valid after login.
	pub fn send_message(&self, chan: &str, msg: &str) -> Result<(),String>{
		let n = self.get_msg_uid();
		let mstr = "{".to_string()+format!(r#""id": {},"type": "message","channel": "{}","text": "{}""#,n,chan,msg).as_slice()+"}";
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err("Failed to get tx!".to_string())
		};
		match tx.send(mstr) {
			Ok(_) => {},
			Err(err) => return Err(format!("{:?}", err))
		}
		Ok(())
	}


	///Runs the main loop for the client after logging in to slack,
	///returns an error if the process fails at an point, or an Ok(()) on succesful
	///close.
	///Takes a MessageHandler (implemented by the user) to call events handlers on.
	///once the first on_receive() or on_ping is called on the MessageHandler, you
	///can soon the 'Only valid after login' methods are safe to use.
	///Sending is run in a thread in parallel while the receive loop runs on the main thread.
	///Both loops should end on return.
	///Sending should be thread safe as the messages are passed in via a channel in
	///RtmClient.send and RtmClient.send_message
	pub fn login_and_run<T: MessageHandler>(&mut self, handler: &mut T, token : &str) -> Result<(),String> {
		//Slack real time api url
		let url = "https://slack.com/api/rtm.start?token=".to_string()+token;

		//Create http client and send request to slack
		let mut client = hyper::Client::new();
		let mut res = match client.get(url.as_slice()).send() {
			Ok(res) => res,
			Err(err) => return Err(format!("Hyper Error: {:?}", err))
		};

		//Read result string
		let res_str = match res.read_to_string() {
			Ok(res_str) => res_str,
			Err(err) => return Err(format!("{:?}", err))
		};









		//Start parsing json. We do not map to a structure,
		//because slack makes no guarantee that there won't be extra fields.
		let js = match Json::from_str(res_str.as_slice()) {
			Ok(js) => js,
			Err(err) => return Err(format!("{:?}", err))
		};

		if !js.is_object() {
			return Err(format!("{} : json is not an object.", RTM_INVALID))
		}
		let jo = js.as_object().unwrap();

		match jo.get("ok") {
			Some(v) => {
				if !(v.is_boolean() && v.as_boolean().unwrap() == true) {
					return Err(format!("{} : js.get(\"ok\") != true : {:?}", RTM_INVALID, jo))
				}
			},
			None => return Err(format!("{} : jo.get(\"ok\") returned None. : {:?}", RTM_INVALID, jo))
		}

		let wss_url_string = match jo.get("url") {
			Some(wss_url) => {
				if wss_url.is_string() {
					wss_url.as_string().unwrap()
				}else{
					return Err(format!("{} : jo.get(\"url\") failed! : {:?}", RTM_INVALID, jo))
				}
			},
			None => return Err(format!("{} : jo.get(\"url\") returned None. : {:?}", RTM_INVALID, jo))
		};

		let wss_url = match Url::parse(wss_url_string) {
			Ok(url) => url,
			Err(err) => return Err(format!("{:?}", err))
		};

		let jself = match jo.get("self") {
			Some(jself) => {
				if jself.is_object() {
					jself.as_object().unwrap()
				}else{
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
		};
		match jself.get("name") {
			Some(jname) => {
				if jname.is_string() {
					self.name = jname.as_string().unwrap().to_string();
				}else{
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
		}
		match jself.get("id") {
			Some(jid) => {
				if jid.is_string() {
					self.id = jid.as_string().unwrap().to_string();
				}else{
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
		}


		let jteam = match jo.get("team") {
			Some(jteam) => {
				if jteam.is_object() {
					jteam.as_object().unwrap()
				}else{
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
		};
		match jteam.get("name") {
			Some(jtname) => {
				if jtname.is_string() {
					self.team.name = jtname.as_string().unwrap().to_string();
				}else{
					return Err(RTM_INVALID.to_string())
				}
			}
			None => return Err(RTM_INVALID.to_string())
		}
		match jteam.get("id") {
			Some(jtid) => {
				if jtid.is_string() {
					self.team.id = jtid.as_string().unwrap().to_string();
				}else{
					return Err(RTM_INVALID.to_string())
				}
			}
			None => return Err(RTM_INVALID.to_string())
		}









		//Make websocket request
		let req = match WebSocketRequest::connect(wss_url) {
			Ok(req) => req,
			Err(err) => return Err(format!("{:?} : WebSocketRequest::connect(wss_url): wss_url{:?}", err, wss_url))
		};

		//Get the key so we can verify it later.
		let key = match req.key() {
			Some(key) => key.clone(),
			None => return Err("Request host key match failed.".to_string())
		};

		//Connect via tls, do websocket handshake.
		let res = match req.send() {
			Ok(res) => res,
			Err(err) => return Err(format!("{:?}, Websocket request to `{:?}` failed", err, wss_url))
		};

		match res.validate(&key) {
			Ok(()) => { }
			Err(err) => return Err(format!("Websocket request key validation error: {:?}", err))
		}

		let mut client = res.begin();

		//for sending messages
		let (tx,rx) = channel::<String>();
		self.outs = Some(tx);

		let mut captured_client = client.clone();

		//websocket send loop
		let guard = Thread::spawn(move || -> () {
			loop {
				let m = match rx.recv() {
					Ok(m) => m,
					Err(err) => panic!(format!("{:?}", err))
				};
			 	let msg = WebSocketMessage::Text(m);
			 	match captured_client.send_message(msg) {
			 		Ok(_) => {},
			 		Err(err) => panic!(format!("{:?}", err))
			 	}
			 }
		});

		let mut sending_client = client.clone();

		//receive loop
		for message in client.incoming_messages() {
			let message = match message {
				Ok(message) => message,
				Err(err) => return Err(format!("{:?}", err))
			};

			match message {
				WebSocketMessage::Text(data) => {
					handler.on_receive(self, data.as_slice());
				},
				WebSocketMessage::Ping(data) => {
					handler.on_ping(self);
					let message = WebSocketMessage::Pong(data);
					match sending_client.send_message(message) {
						Ok(_) => {},
						Err(err) => { return Err(format!("{:?}", err)); }
					}
				},
				WebSocketMessage::Close(data) => {
					handler.on_close(self);
					let message = WebSocketMessage::Close(data);
					match sending_client.send_message(message) {
						Ok(_) => {},
						Err(err) => { return Err(format!("{:?}", err)); }
					}
					return Ok(());
				},
				_ => {}
			}
		}

		Ok(())
	}
}
