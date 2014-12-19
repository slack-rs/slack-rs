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
extern crate slack;
extern crate time;


struct MyHandler {
	count : int
}

#[allow(unused_variables)]
impl slack::MessageHandler for MyHandler {
	fn on_receive(&mut self, cli: &mut slack::RtmClient, json_str: &str){
		println!("{} | Received[{}]: {}", time::now().rfc822z(), self.count, json_str.to_string())
		self.count = self.count + 1;
	}

	fn on_ping(&mut self, cli: &mut slack::RtmClient){
		println!("{} | <on_ping>", time::now().rfc822z())
	}

	fn on_close(&mut self, cli: &mut slack::RtmClient){
		println!("{} | <on_close>", time::now().rfc822z())
	}
}

fn main(){
	let args = std::os::args();
	let api_key = match args.capacity() {
		0 | 1 => panic!("No api-key in args! Usage: ./edwin <api-key>"),
		x => {
			let i = x-1;
			args.index(&i)
		}
	};
	let mut handler = MyHandler{count: 0};
	let mut cli = slack::RtmClient::new();
	let r = cli.login_and_run::<MyHandler>(&mut handler, api_key.as_slice());
	match r {
		Ok(_) => {},
		Err(err) => println!("Error: {}", err)
	}
	println!("{}", cli.get_name());
	println!("{}", cli.get_team().get_name());
}
