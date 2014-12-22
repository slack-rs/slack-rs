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
extern crate serialize;
extern crate openssl;

use serialize::json;
use std::io::TcpStream;
use std::comm::channel;
use std::thread::Thread;
use std::boxed::BoxAny;
use std::sync::atomic::{AtomicInt, SeqCst};
use websocket::{WebSocketClient,WebSocketClientMode};
use websocket::message::WebSocketMessage;
use websocket::handshake::WebSocketRequest;
use openssl::ssl::{SslContext,SslMethod,SslStream};

type WebsocketSslClient = WebSocketClient<SslStream<TcpStream>>;

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

		let wss_url = match jo.get("url") {
			Some(wss_url) => {
				if wss_url.is_string() {
					wss_url.as_string().unwrap()
				}else{
					return Err(RTM_INVALID.to_string())
				}
			},
			None => return Err(RTM_INVALID.to_string())
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
		let mut req = match WebSocketRequest::new(wss_url, [""].as_slice()) {
			Ok(req) => req,
			Err(err) => return Err(format!("{}", err))
		};

		//Slack doesn't support or use these:
		req.headers.remove("sec-websocket-protocol");

		//Get the key so we can verify it later.
		let key = match req.key() {
			Some(key) => key,
			None => return Err("Request host key match failed.".to_string())
		};

		//Get the host for the tcp stream.
		let host = match req.host() {
			Some(host) => host,
			None => return Err("Failed to get host.".to_string())
		};

		//Connect
		let connection = match TcpStream::connect(host.as_slice()) {
			Ok(connection) => connection,
			Err(err) => return Err(format!("{}", err))
		};

		//Get an openssl tls context
		let ssl_ctx = match SslContext::new(SslMethod::Tlsv1){
			Ok(ssl_ctx) => ssl_ctx,
			Err(err) => return Err(format!("{}", err))
		};
		

		//Connect via tls, do websocket handshake.
		let client = match SslStream::new(&ssl_ctx, connection) {
			Ok(stream) => {
				let mut client = WebSocketClient::new(stream, WebSocketClientMode::RemoteServer);
				match client.send_handshake_request(&req) {
					Ok(_) => {},
					Err(err) => return Err(format!("{}", err))
				}
				let response = match client.receive_handshake_response() {
					Ok(response) => response,
					Err(err) => return Err(format!("{}", err))
				};
				//FIXME: come back to this, right now this seems to be giving false positives.
				/*if !response.is_successful(key) {
					return Err("Failed handshake.".to_string());
				}*/
				client
			},
			Err(err) => return Err(format!("{}", err))
		};



		//for sending messages
		let (tx,rx) = channel::<String>();
		self.outs = Some(tx);

		let captured_client = client.clone(); 

		//websocket send loop
		let guard = Thread::spawn(move || -> () {
			let mut sender = captured_client.sender(); 
			loop {
				let m = match rx.recv_opt() {
					Ok(m) => m,
					Err(err) => panic!(format!("{}", err))
				};
			 	let msg = WebSocketMessage::Text(m);
			 	match sender.send_message(&msg) {
			 		Ok(_) => {},
			 		Err(err) => panic!(format!("{}", err))
			 	}
			 }
		});


		let mut sender = client.sender();
		//receive loop
		for message in client.receiver().incoming() {
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
					match sender.send_message(&message) {
						Ok(_) => {},
						Err(err) => { return Err(format!("{}", err)); }
					}
				},
				WebSocketMessage::Close(data) => {
					handler.on_close(self);
					let message = WebSocketMessage::Close(data);
					match sender.send_message(&message) {
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
