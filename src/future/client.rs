use {url, reqwest, tokio_core, Error, Event, serde_json, WsMessage};
use std::sync::atomic::{AtomicUsize, Ordering};
use api::{self, Channel, Group, User, Team, Im};
use futures::sync::mpsc;
use futures::{Future, Stream};
use futures::future::{err, ok};
use tokio_core::net::TcpStream;
use native_tls::TlsConnector;
use tokio_tls::TlsConnectorExt;
use std::net::ToSocketAddrs;
use std::collections::HashMap;
use tungstenite::Message;
use tokio_tungstenite::client_async;
use std::boxed::Box;
use std::sync::Arc;
use std::mem;

/// The slack messaging client.
pub struct Client {
    token: String,
    start_info: Option<api::rtm::StartResponse>,
    channels: Vec<Channel>,
    groups: Vec<Group>,
    users: Vec<User>,
    channel_ids: HashMap<String, String>,
    group_ids: HashMap<String, String>,
    user_ids: HashMap<String, String>,
    rx: Option<mpsc::UnboundedReceiver<WsMessage>>,
    sender: Option<Sender>,
}

/// Thread-safe API for sending messages asynchronously
#[derive(Clone)]
pub struct Sender {
    tx: mpsc::UnboundedSender<WsMessage>,
    msg_num: Arc<AtomicUsize>,
}

impl_sender!();

/// Implement this trait in your code to handle message events
pub trait EventHandler {
    /// When a message is received this will be called with self, the slack client,
    /// and the result of parsing the event received, as well as the raw json string.
    fn on_event(&mut self,
                _cli: &mut Client,
                _event: Result<Event, Error>,
                _raw_json: &str)
                -> Box<Future<Item = (), Error = ()>> {
        Box::new(ok(()))
    }

    /// Called when the connection is closed for any reason.
    fn on_close(&mut self, _cli: &mut Client) -> Box<Future<Item = (), Error = ()>> {
        Box::new(ok(()))
    }

    /// Called when the connection is opened.
    fn on_connect(&mut self, _cli: &mut Client) -> Box<Future<Item = (), Error = ()>> {
        Box::new(ok(()))
    }
}

/// Like `try!` but for a future
#[macro_export]
macro_rules! try_fut {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Box::new(err(e.into())),
        }
    }
}

impl Client {
    /// Creates a new client from a token
    pub fn new(token: &str) -> Client {
        Client {
            token: String::from(token),
            start_info: None,
            channels: Vec::new(),
            groups: Vec::new(),
            users: Vec::new(),
            channel_ids: HashMap::new(),
            group_ids: HashMap::new(),
            user_ids: HashMap::new(),
            rx: None,
            sender: None,
        }
    }

    client_common_non_blocking!();

    /// Login to slack and get the websocket url (needed for calling `run`)
    pub fn login(&mut self) -> Result<reqwest::Url, Error> {
        let client = reqwest::Client::new()?;
        let start = try!(api::rtm::start(&client, &self.token, &Default::default()));
        let start_url = &start.url.clone().expect("websocket url from slack");
        let wss_url = reqwest::Url::parse(&start_url)?;

        if let Some(ref channels) = start.channels {
            for ref channel in channels.iter() {
                self.channel_ids
                    .insert(channel.name.clone().unwrap(), channel.id.clone().unwrap());
            }
            self.channels = channels.clone();
        }
        if let Some(ref groups) = start.groups {
            for ref group in groups.iter() {
                self.group_ids
                    .insert(group.name.clone().unwrap(), group.id.clone().unwrap());
            }
            self.groups = groups.clone();
        }

        if let Some(ref users) = start.users {
            for ref user in users.iter() {
                self.user_ids
                    .insert(user.name.clone().unwrap(), user.id.clone().unwrap());
            }
            self.users = users.clone();
        }

        let (tx, rx) = mpsc::unbounded();
        let sender = Sender {
            tx: tx,
            msg_num: Arc::new(AtomicUsize::new(0)),
        };
        self.sender = Some(sender);
        self.rx = Some(rx);

        // store rtm.Start data
        self.start_info = Some(start);
        Ok(wss_url)
    }

    /// Run a non-blocking slack client
    // XXX: once `impl Trait` is stabilized we can get rid of all of these `Box`es
    pub fn run<'a, T: EventHandler>(&'a mut self,
                                    handler: &'a mut T,
                                    wss_url: reqwest::Url,
                                    handle: &tokio_core::reactor::Handle)
                                    -> Box<Future<Item = (), Error = Error> + 'a> {
        let addr = match try_fut!(wss_url.to_socket_addrs()).next() {
            None => return Box::new(err(Error::Internal("Websocket addr not found".into()))),
            Some(a) => a,
        };

        let rx = match self.rx {
            None => return Box::new(err(Error::Internal("Receiver missing. You must call `login` before `run`.".into()))),
            Some(_) => mem::replace(&mut self.rx, None).unwrap(),
        };

        let domain = match wss_url.origin() {
            url::Origin::Tuple(_, domain, _) => {
                match domain {
                    url::Host::Domain(d) => d,
                    s => {
                        return Box::new(err(Error::Internal(format!("Expected domain, found: {:?}",
                                                                    s))));
                    }
                }
            }
            s => return Box::new(err(Error::Internal(format!("Expected Origin {:?}", s)))),
        };
        let socket = TcpStream::connect(&addr, handle);
        let cx = try_fut!(try_fut!(TlsConnector::builder()).build());
        let tls_handshake =
            socket
                .map_err(Error::from)
                .and_then(move |socket| cx.connect_async(&domain, socket).map_err(Error::from));

        let stream =
            tls_handshake
                .map_err(Error::from)
                .and_then(move |stream| client_async(wss_url, stream).map_err(Error::from));

        let client = stream
            .and_then(move |ws_stream| {
                handler
                    .on_connect(self)
                    .map_err(Error::from)
                    .and_then(move |_| {
                        let (mut sink, stream) = ws_stream.split();
                        let ws_reader = stream
                            .map_err(Error::from)
                            .and_then(move |message| match message {
                                          Message::Text(text) => {
                                              match Event::from_json(&text[..]) {
                                                  Ok(event) => {
                                                      Box::new(handler
                                                                   .on_event(self,
                                                                             Ok(event),
                                                                             &text)
                                                                   .map_err(|_| Error::Unit)) as
                                                      Box<Future<Item = (), Error = Error>>
                                                  }
                                                  Err(err) => {
                                                      Box::new(handler
                                                                   .on_event(self,
                                                                             Err(err),
                                                                             &text)
                                                                   .map_err(|_| Error::Unit)) as
                                                      Box<Future<Item = (), Error = Error>>
                                                  }
                                              }
                                          }
                                          Message::Binary(_) => Box::new(ok::<(), Error>(())),
                                      })
                            .for_each(|_| Ok(()));

                        let ws_writer = rx.take_while(|msg| match *msg {
                                                          WsMessage::Close => Ok(false),
                                                          _ => Ok(true),
                                                      })
                            .for_each(move |msg| {
                                use futures::Sink;
                                match msg {
                                    WsMessage::Text(text) => {
                                        if sink.start_send(Message::Text(text)).is_err() {
                                            return Err(());
                                        }
                                    }
                                    WsMessage::Close => unreachable!(),
                                }
                                Ok(())
                            })
                            .map_err(|_| Error::Unit)
                            .map(|_| ());

                        Box::new(ws_reader
                                     .select(ws_writer)
                                     .then(|res| match res {
                                               Ok(_) => Ok(()),
                                               Err((a, _)) => Err(a.into()),
                                           })) as
                        Box<Future<Item = (), Error = Error>>
                    })
            })
            .map_err(Error::from);
        Box::new(client)
    }

    /// Send a shutdown message to close the connection to slack
    pub fn shutdown(&self) -> Result<(), Error> {
        match self.sender {
            Some(ref sender) => {
                (&sender.tx)
                    .send(WsMessage::Close)
                    .map_err(|_| Error::Internal("Sending shutdown message failed".into()))
            }
            None => {
                Err(Error::Internal("Cannot shutdown without a sender. Ensure you have run `login`."
                                        .into()))
            }
        }
    }
}
