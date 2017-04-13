extern crate slack;
extern crate futures;
extern crate tokio_core;

#[cfg(feature = "future")]
fn main() {
    use slack::{Event, Message};
    use slack::api::MessageStandard;
    use slack::future::client::{Client, EventHandler};
    use std::boxed::Box;
    use futures::Future;
    use futures::future::ok;

    pub struct MyHandler;

    impl EventHandler for MyHandler {
        fn on_event(&mut self,
                    _cli: &mut Client,
                    event: ::std::result::Result<Event, slack::Error>,
                    _raw_json: &str)
                    -> Box<Future<Item = (), Error = ()>> {
            if let Ok(event) = event {
                println!("event = {:#?}", event);
                // do something if we get a message event
                if let Event::Message(ref message) = event {
                    if let Message::Standard(MessageStandard {
                                                 ref channel,
                                                 ref user,
                                                 ref text,
                                                 ref ts,
                                                 ..
                                             }) = **message {
                        println!("{:?}, {:?}, {:?}, {:?}", channel, user, text, ts);
                    }
                }
            }
            Box::new(ok(()))
        }

        fn on_close(&mut self, _cli: &mut Client) -> Box<Future<Item = (), Error = ()>> {
            println!("on_close");
            Box::new(ok(()))
        }

        fn on_connect(&mut self, _cli: &mut Client) -> Box<Future<Item = (), Error = ()>> {
            println!("on_connect");
            Box::new(ok(()))
        }
    }

    let token = "REPLACE_ME";
    let mut client = Client::new(token);
    let wss_url = client.login().unwrap();
    let mut my_handler = MyHandler;
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let fut = client.run(&mut my_handler, wss_url, &core.handle());

    core.run(fut).unwrap();
}

#[cfg(not(feature = "future"))]
fn main() {}
