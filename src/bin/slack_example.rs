/*
Copyright 2014 Benjamin Elder from https://github.com/BenTheElder/slack-rs-demo

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

/*
    This is a simple example of using slack-rs.
    You can run it with `cargo run example -- <api_key>`
*/

extern crate slack;


struct MyHandler {
	count : i64
}

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
	fn on_receive(&mut self, cli: &mut slack::RtmClient, json_str: &str){
		println!("Received[{}]: {}", self.count, json_str.to_string());
		self.count = self.count + 1;
	}

	fn on_ping(&mut self, cli: &mut slack::RtmClient){
		println!("<on_ping>");
	}

	fn on_close(&mut self, cli: &mut slack::RtmClient){
		println!("<on_close>");
	}

	fn on_connect(&mut self, cli: &mut slack::RtmClient){
		println!("<on_connect>");
		// Do a few things using the api:
		// send a message over the real time api websocket
		let _ = cli.send_message("#general", "Hello world! (rtm)");
		// post a message as a user to the web api
		let _ = cli.post_message("#general", "hello world! (postMessage)", None);
		// set a channel topic via the web api
		let _ = cli.set_topic("#general", "bots rule!");
	}
}

fn main(){
  	let args: Vec<String> = std::env::args().collect();
  	let api_key = match args.len() {
    	0 | 1 => panic!("No api-key in args! Usage: ./slack-demo <api-key>"),
    	x => {
      		args[x-1].clone()
    	}
  	};
  	let mut handler = MyHandler{count: 0};
  	let mut cli = slack::RtmClient::new(&api_key);
  	let r = cli.login_and_run::<MyHandler>(&mut handler);
  	match r {
    	Ok(_) => {},
    	Err(err) => panic!("Error: {}", err)
	}
	println!("{}", cli.get_name().unwrap());
	println!("{}", cli.get_team().unwrap().name);
}
