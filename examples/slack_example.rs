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
//
//
// This is a simple example of using slack-rs.
// You can run it with `cargo run --example slack_example -- <api_key>`
//
// NOTE: Only classic app bots can connect to rtm https://api.slack.com/rtm#classic
//
// NOTE: This will post in the #general channel of the account you connect
// to.
//

use slack;
use slack::{Event, RtmClient};

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        if let Event::Hello = event {
            // find the general channel id from the `StartResponse`
            let general_channel_id = cli
                .start_response()
                .channels
                .as_ref()
                .and_then(|channels| {
                    channels.iter().find(|chan| match chan.name {
                        None => false,
                        Some(ref name) => name == "general",
                    })
                })
                .and_then(|chan| chan.id.as_ref())
                .expect("general channel not found");
            let _ = cli
                .sender()
                .send_message(&general_channel_id, "Hello world! (rtm)");
            // Send a message over the real time api websocket
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => {
            panic!("No api-key in args! Usage: cargo run --example slack_example -- <api-key>")
        }
        x => args[x - 1].clone(),
    };
    let mut handler = MyHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
