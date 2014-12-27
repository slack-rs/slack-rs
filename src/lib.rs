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

use rustc_serialize::json;
use std::comm::channel;
use std::thread::Thread;
use std::boxed::BoxAny;
use std::sync::atomic::{AtomicInt, SeqCst};
use websocket::message::WebSocketMessage;
use websocket::handshake::WebSocketRequest;
use url::Url;

pub struct Team {
	name : String,
	id : String
}

impl Team {
	fn new() -> Team {
		Team{name: "".to_string(), id: "".to_string()}
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}
}


pub trait MessageHandler {
	fn on_receive(&mut self, cli: &mut RtmClient, json_str: &str);
	fn on_ping(&mut self, cli: &mut RtmClient);
	fn on_close(&mut self, cli: &mut RtmClient);
}

pub struct RtmClient {
	name : String,
	id : String,
	team : Team,
	msg_num: AtomicInt,
	outs : Option<Sender<String>>
}

static RTM_INVALID : &'static str = "Invalid data returned from slack (rtm.start)";

impl RtmClient {

	pub fn new() -> RtmClient {
		RtmClient{
			name : "".to_string(),
			id : "".to_string(),
			team : Team::new(),
			msg_num: AtomicInt::new(0),
			outs : None
		}
	}

	pub fn get_name(&self) -> String {
		return self.name.clone();
	}

	pub fn get_id(&self) -> String {
		return self.id.clone();
	}

	pub fn get_team<'a>(&'a self) -> &'a Team {
		&self.team
	}


	pub fn send(&mut self, s : &str) -> Result<(),String> {
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err("Failed to get tx!".to_string())
		};
		match tx.send_opt(s.to_string()) {
			Ok(_) => {},
			Err(err) => return Err(format!("{}", err))
		}
		Ok(())
	}

	pub fn send_message(&self, chan: &str, msg: &str) -> Result<(),String>{
		let n = self.msg_num.fetch_add(1, SeqCst);
		let mstr = "{".to_string()+format!(r#""id": {},"type": "message","channel": "{}","text": "{}""#,n,chan,msg).as_slice()+"}";
		let tx = match self.outs {
			Some(ref tx) => tx,
			None => return Err("Failed to get tx!".to_string())
		};
		match tx.send_opt(mstr) {
			Ok(_) => {},
			Err(err) => return Err(format!("{}", err))
		}
		Ok(())
	}

	pub fn login_and_run<T: MessageHandler>(&mut self, handler: &mut T, token : &str) -> Result<(),String> {
		//Slack real time api url
		let url = "https://slack.com/api/rtm.start?token=".to_string()+token;
		
		//Create http client and send request to slack
		let mut client = hyper::Client::new();
		let mut res = match client.get(url.as_slice()).send() {
			Ok(res) => res,
			Err(err) => return Err(format!("Hyper Error: {}", err))
		};

		//Read result string
		let res_str = match res.read_to_string() {
			Ok(res_str) => res_str,
			Err(err) => return Err(format!("{}", err))
		};









		//Start parsing json. We do not map to a structure,
		//because slack makes no guarantee that there won't be extra fields.
		let js = match json::from_str(res_str.as_slice()) {
			Ok(js) => js,
			Err(err) => return Err(format!("{}", err))
		};

		if !js.is_object() {
			return Err(RTM_INVALID.to_string())
		}
		let jo = js.as_object().unwrap();

		match jo.get("ok") {
			Some(v) => { 
				if !(v.is_boolean() && v.as_boolean().unwrap() == true) {
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
		}

		let wss_url_string = match jo.get("url") {
			Some(wss_url) => {
				if wss_url.is_string() {
					wss_url.as_string().unwrap()
				}else{
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
		};

		let wss_url = match Url::parse(wss_url_string) {
			Ok(url) => url, 
			Err(err) => return Err(format!("{}", err))
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
			Err(err) => return Err(format!("{}", err))
		};

		//Get the key so we can verify it later.
		let key = match req.key() {
			Some(key) => key.clone(),
			None => return Err("Request host key match failed.".to_string())
		};

		//Connect via tls, do websocket handshake.
		let res = match req.send() {
			Ok(res) => res,
			Err(err) => return Err(format!("{}", err))
		};

		match res.validate(&key) {
			Ok(()) => { }
			Err(err) => return Err(format!("{}", err))
		}
		
		let mut client = res.begin();

		//for sending messages
		let (tx,rx) = channel::<String>();
		self.outs = Some(tx);

		let mut captured_client = client.clone(); 

		//websocket send loop
		let guard = Thread::spawn(move || -> () {
			loop {
				let m = match rx.recv_opt() {
					Ok(m) => m,
					Err(err) => panic!(format!("{}", err))
				};
			 	let msg = WebSocketMessage::Text(m);
			 	match captured_client.send_message(msg) {
			 		Ok(_) => {},
			 		Err(err) => panic!(format!("{}", err))
			 	}
			 }
		});

		let mut sending_client = client.clone();
		
		//receive loop
		for message in client.incoming_messages() {
			let message = match message {
				Ok(message) => message,
				Err(err) => return Err(format!("{}", err))
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
						Err(err) => { return Err(format!("{}", err)); }
					}
				},
				WebSocketMessage::Close(data) => {
					handler.on_close(self);
					let message = WebSocketMessage::Close(data);
					match sending_client.send_message(message) {
						Ok(_) => {},
						Err(err) => { return Err(format!("{}", err)); }
					}
					return Ok(());
				},
				_ => {}
			}
		}

		match guard.join() {
			Err(err) => {
				Err(*err.downcast::<String>().unwrap())
			},
			Ok(_) => Ok(())
		}
	}
}
