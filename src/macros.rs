//! Macros

/// Common functions across both the sync and futures-based clients. These functions are all
/// non-blocking.
macro_rules! client_common_non_blocking {
    () => {
        /// Returns the Team struct of the bot/user connected to the client.
        /// Only valid after login, otherwise None.
        pub fn get_team(&self) -> Option<Team> {
            match self.start_info {
                Some(ref s) => s.team.clone(),
                None => None,
            }
        }

        /// Get a user id from a username
        /// Only valid after login.
        pub fn get_user_id(&self, username: &str) -> Option<&String> {
            self.user_ids.get(username)
        }

        /// Evaluate if chan is a channel name or channel id
        /// If channel name, returns its id
        /// If channel id, returns itself
        /// Only valid after login.
        fn evaluate_channel_id(&self, chan: &str) -> Result<String, Error> {
            let id = if chan.starts_with('#') {
                match self.get_channel_id(&chan[1..]) {
                    Some(s) => s,
                    None => return Err(Error::Internal(String::from("need to login first to retrieve channel list"))),
                }
            } else {
                chan
            };

            Ok(id.to_string())
        }

        /// Get a channel id from a channel name, note that channel_name does not begin with a '#'
        /// Only valid after login.
        pub fn get_channel_id(&self, channel_name: &str) -> Option<&String> {
            self.channel_ids.get(channel_name)
        }

        /// Get a group id from a group name
        /// Only valid after login.
        pub fn get_group_id(&self, group_name: &str) -> Option<&String> {
            self.group_ids.get(group_name)
        }

        /// Returns a vector of Users from the team the bot/client is connected to.
        /// Only valid after login.
        pub fn get_users(&self) -> Vec<User> {
            self.users.clone()
        }

        /// Returns a vector of Channels from the team the bot/client is connected to.
        /// Only valid after login.
        pub fn get_channels(&self) -> Vec<Channel> {
            self.channels.clone()
        }

        /// Returns a vector of Groups from the team the bot/client is connected to.
        /// Only valid after login.
        pub fn get_groups(&self) -> Vec<Group> {
            self.groups.clone()
        }

        /// Returns a vector of Ims received on login the bot/client is connected to.
        /// Only valid after login, otherwise None.
        pub fn get_start_ims(&self) -> Option<Vec<Im>> {
            match self.start_info {
                Some(ref s) => s.ims.clone(),
                None => None,
            }
        }

        ///Returns a unique identifier to be used in the 'id' field of a message
        ///sent to slack.
        pub fn get_msg_uid(&self) -> usize {
            self.sender
                .as_ref()
                .unwrap()
                .msg_num
                .fetch_add(1, Ordering::SeqCst)
        }

        /// Get a thread-safe message sender
        pub fn channel(&self) -> Option<Sender> {
            self.sender.clone()
        }

        /// Allows sending a json string message over the websocket connection.
        /// Note that this only passes the message over a channel to the
        /// Messaging task, and therefore a successful return value does not
        /// mean the message has been actually put on the wire yet.
        /// Note that you will need to form a valid json reply yourself if you
        /// use this method, and you will also need to retrieve a unique id for
        /// the message via RtmClient.get_msg_uid()
        /// Only valid after login.
        pub fn send(&mut self, s: &str) -> Result<(), Error> {
            let tx = match self.sender {
                Some(ref sender) => &sender.tx,
                None => return Err(Error::Internal(String::from("Failed to get tx!"))),
            };
            tx.send(WsMessage::Text(s.to_string()))
                .map_err(|err| Error::Internal(format!("{}", err)))?;
            Ok(())
        }

        /// Allows sending a textual string message over the websocket connection,
        /// to the requested channel id. Ideal usage would be EG:
        /// extract the channel in on_receive and then send back a message to the channel.
        /// Note that this only passes the message over a channel to the
        /// Messaging task, and therefore a successful return value does not
        /// mean the message has been actually put on the wire yet.
        /// This method also handles getting a unique id and formatting the actual json
        /// sent.
        /// Only valid after login.
        pub fn send_message(&self, chan: &str, msg: &str) -> Result<usize, Error> {
            let n = self.get_msg_uid();

            let chan_id = match self.evaluate_channel_id(chan) {
                Ok(id) => id,
                _ => return Err(Error::Internal(String::from("Failed to get channel id"))),
            };

            let msg_json = serde_json::to_string(&msg)?;
            let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                               n,
                               chan_id,
                               &msg_json[1..msg_json.len() - 1]);
            let tx = match self.sender {
                Some(ref sender) => &sender.tx,
                None => return Err(Error::Internal(String::from("Failed to get tx!"))),
            };
            tx.send(WsMessage::Text(mstr))
                .map_err(|err| Error::Internal(format!("{:?}", err)))?;
            Ok(n)
        }

        /// Marks connected client as being typing to a channel
        /// This is mostly used to signal to other peers that a message
        /// is being typed. Will have the server send a "user_typing" message to all the
        /// peers.
        /// Slack doc can be found at https://api.slack.com/rtm under "Typing Indicators"
        pub fn send_typing(&self, chan: &str) -> Result<usize, Error> {
            let n = self.get_msg_uid();

            let chan_id = match self.evaluate_channel_id(chan) {
                Ok(id) => id,
                _ => return Err(Error::Internal(String::from("Failed to get channel id"))),
            };

            let mstr = format!(r#"{{"id": {}, "type": "typing", "channel": "{}"}}"#,
                               n,
                               chan_id);

            let tx = match self.sender {
                Some(ref sender) => &sender.tx,
                None => return Err(Error::Internal(String::from("Failed to get tx!"))),
            };

            tx.send(WsMessage::Text(mstr))
                .map_err(|err| Error::Internal(format!("{:?}", err)))?;
            Ok(n)
        }
    }
}

/// Common `Sender` implementation across sync and futures-based clients
macro_rules! impl_sender {
    () => {
        impl Sender {
            /// Get the next message id
            ///
            /// A value returned from this method *must* be included in the JSON payload
            /// (the `id` field) when constructing your own message.
            pub fn get_msg_uid(&self) -> usize {
                self.msg_num.fetch_add(1, Ordering::SeqCst)
            }

            /// Send a raw message
            ///
            /// Must set `message.id` using result of `get_msg_id()`.
            ///
            /// Success from this API does not guarantee the message is delivered
            /// successfully since that runs on a separate task.
            pub fn send(&self, raw: &str) -> Result<(), Error> {
                self.tx.send(WsMessage::Text(raw.to_string()))
                         .map_err(|err| Error::Internal(format!("{}", err)))?;
                Ok(())
            }

            /// Send a message to the specified channel id
            ///
            /// Success from this API does not guarantee the message is delivered
            /// successfully since that runs on a separate task.
            pub fn send_message_chid(&self, chan_id: &str, msg: &str) -> Result<usize, Error> {
                let n = self.get_msg_uid();
                let msg_json = serde_json::to_string(&msg)?;
                let mstr = format!(r#"{{"id": {},"type": "message", "channel": "{}","text": "{}"}}"#,
                                   n,
                                   chan_id,
                                   &msg_json[1..msg_json.len() - 1]);

                self.send(&mstr[..])?;
                Ok(n)
            }
        }
    }
}
