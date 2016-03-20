# Changelog

## 0.13.0
- Thanks to https://github.com/squidpickles and https://github.com/dten respectively: add unnoficial events to handle message sending success and error, and add timeout on the rtm sockeckt.

### Compatibility Changes
- Two new Events: `MessageSent` and `MessageError`
- The RtmClient now has a 70 second timeout on the socket. This will be adjusted in the future.

## 0.12.2
- Thanks to https://github.com/squidpickles handle new reconnect_url events, and rewrite event API to use match instead of if/else branches. Also change dev-dependencies to specific versions to comply with crates.io

## 0.12.1
- overhaul websocket teardown logic.

## 0.12.0
- Overhauled event parsing and added Event type, updated the EventHandler api, updated dependencies, added Error::Utf8 for utf8 decoding errors, rustfmt-ed the sources, various bugfixes.

### Compatibility Changes
- EventHandler's on_receive is now on_event with a different signature that takes the raw json string as well as the result of parsing the Event, for less library-user parsing and greater flexibility.
- RtmClient's get_outs method has been removed, and the type of the channel used for passing messages between the working threads has changed.


## 0.11.0
- Bugfix changes the color field of User to `Option<String>`, see: https://github.com/BenTheElder/slack-rs/issues/22

## 0.10.1
- Massive overhaul, implement support for almost all of the bots api, stronger error handling and lots of tests. Thanks a ton to https://github.com/mthjones, see https://github.com/BenTheElder/slack-rs/pull/17 for the main overhaul.

### Compatibility Changes
Methods that previously returned `Result<String,Error>` now return a typed `Result<Some_Slack_Response_Type, Error>`:

- `RtmClient::post_message` now returns `Result<api::chat::PostMessageResponse, Error>`
- `RtmClient::delete_message` now returns `Result<api::chat::DeleteResponse, Error>`
- `RtmClient::mark` now returns `Result<api::channels::MarkResponse, Error>`
- `RtmClient::set_topic` now returns `Result<api::channels::SetTopicResponse, Error>`
- `RtmClient::set_purpose` now returns `Result<api::channels::SetPurposeResponse, Error>`
- `RtmClient::add_reaction_timestamp` now returns `Result<api::reactions::AddResponse, Error>`
- `RtmClient::add_reaction_file` now returns `Result<api::reactions::AddResponse, Error>`
- `RtmClient::add_reaction_file_comment` now returns `Result<api::reactions::AddResponse, Error>`
- `RtmClient::update_message` now returns `Result<api::chat::UpdateResponse, Error>`
- `RtmClient::im_open` now returns `Result<api::im::OpenResponse, Error>`
- `RtmClient::channels_history` now returns `Result<api::channels::HistoryResponse, Error>`
- `RtmClient::im_close` now returns `Result<api::im::CloseResponse, Error>`
- `RtmClient::im_history` now returns `Result<api::im::HistoryResponse, Error>`
- `RtmClient::im_list` now returns `Result<api::im::ListResponse, Error>`
- `RtmClient::im_mark` now returns `Result<api::im::MarkResponse, Error>`

Forthcoming releases will see the implementation of the remaining files.upload and some convenient helpers such as a message builder can be expected in a later release, and the Error::Api will expose Slack api error types more strongly in a forthcoming release.

## 0.9.2
- Add channels_history via https://github.com/jeehoonkang https://github.com/BenTheElder/slack-rs/pull/16

## 0.9.1
- With help from: https://github.com/mthjones, overhaul error handling and refactor, improve api support.
- Introduced slack::error::Error
- Added a number of bots api methods
- Fixed bug where setPurpose called setTopic instead [!]

## 0.8.3
- Moved example to examples dir thanks to https://github.com/mthjones: https://github.com/BenTheElder/slack-rs/pull/9

## 0.8.2
- Fix https://github.com/BenTheElder/slack-rs/issues/8

## 0.8.1
- Add some web api methods, add methods to map names to ids.
- TODO: expect the error type overhaul to be pushed back to 0.9.X

### Compatibility Changes
- RtmClient::new now takes the bot token/api_key and login, login_and_run do not.

## 0.7.2
- Bugfix via https://github.com/Farthen: https://github.com/BenTheElder/slack-rs/pull/6

## 0.7.1
- Cleaned up the api and json handling.
- TODO: expect better error handling in 0.8.X

### Compatibility Changes
- 'MessageHandler' is now 'EventHandler' and all of the slack data structs have been updated to match the api as closely as possible.

## 0.6.1
- Updated to stable rust.