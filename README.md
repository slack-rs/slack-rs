# slack-rs

[Slack][slack] realtime messaging client and API interface.

[![Build Status][ci-img]][ci-url] [![License][license-img]][license-url] [![Crates.io][crates-img]][crates-url]

[Documentation](https://slack-rs.github.io/slack-rs)

# Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
slack = "0.25.0"
```

and this to your crate root:

```rust
extern crate slack;
```

# Example
See the [examples directory](./examples).

# License
`slack-rs` is distributed under the [Apache-2.0 License](./LICENSE).

[ci-img]: https://travis-ci.org/slack-rs/slack-rs.svg?branch=master
[ci-url]: https://travis-ci.org/slack-rs/slack-rs
[crates-img]: https://img.shields.io/crates/v/slack.svg
[crates-url]: https://crates.io/crates/slack
[license-img]: https://img.shields.io/github/license/slack-rs/slack-rs.svg
[license-url]: https://raw.githubusercontent.com/slack-rs/slack-rs/master/LICENSE
[slack]: https://api.slack.com/
