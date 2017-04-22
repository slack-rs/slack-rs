extern crate slack;
extern crate futures;
extern crate tokio_core;

#[cfg(feature = "future")]
fn main() {
    use slack::{Event, Message};
    use slack::api::MessageStandard;
    use slack::future::client::{Client, EventHandler};
    use futures::future::{ok, FutureResult};

    struct MyHandler;

    impl EventHandler for MyHandler {
        type EventFut = FutureResult<(), ()>;
        type OnCloseFut = FutureResult<(), ()>;
        type OnConnectFut = FutureResult<(), ()>;

        fn on_event(&mut self,
                    _cli: &mut Client,
                    event: Result<Event, slack::Error>)
                    -> Self::EventFut {
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
            ok(())
        }

        fn on_close(&mut self, _cli: &mut Client) -> Self::OnCloseFut {
            println!("on_close");
            ok(())
        }

        fn on_connect(&mut self, _cli: &mut Client) -> Self::OnConnectFut {
            println!("on_connect");
            ok(())
        }
    }

    let token = "REPLACE_ME";
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    core.run(Client::connect(token, MyHandler, &handle))
        .unwrap();
}

#[cfg(not(feature = "future"))]
fn main() {}
