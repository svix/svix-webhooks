<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">Svix - Webhooks as a service</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://svix.com">Website</a> | <a href="https://docs.svix.com">Documentation</a> | <a href="https://svix.com/slack">Community Slack</a>
<h2>

Rust library for interacting with the Svix API and verifying webhook signatures

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![Crates.io](https://img.shields.io/crates/v/svix)](https://crates.io/crates/svix)
[![docs.rs](https://docs.rs/svix/badge.svg)](https://docs.rs/svix/)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

# Usage Documentation

You can find general usage documentation at <https://docs.svix.com>.  For complete API documentation with code examples for each endpoint in all of our official client libraries head over to our API documentation site at <https://api.svix.com>.

# Language Support

<table style="table-layout:fixed; white-space: nowrap;">
  <th colspan="2">⚡️ Features ⚡️</th>
  <tr>
    <th>Officially Supported</th>
    <th>✅</th>
  </tr>
  <tr>
    <th>API Support</th>
    <th>✅</th>
  </tr>
  <tr>
    <th>Signature Verification</th>
    <th>✅</th>
  </tr>
  <tr>
    <th>Caveats</th>
    <th>None! 🚀</th>
  </tr>
</table>

## Usage
Please refer to [the documentation](https://docs.svix.com/) or [the API reference](https://api.svix.com/docs) for more usage instructions.

# Optional Cargo Features

## TLS

By default reqwest uses rust-native-tls, which will use the operating system TLS framework if available, meaning Windows and macOS. On Linux, it will use OpenSSL 1.1.

[rustls-tls](https://github.com/rustls/rustls) can also be choosen, eg.:
```
svix = { version = "N", features = ["rustls-tls"], default-features = false  }
```

