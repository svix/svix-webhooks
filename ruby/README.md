<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">Svix - Webhooks as a service</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://svix.com">Website</a> | <a href="https://docs.svix.com">Documentation</a> | <a href="https://svix.com/slack">Community Slack</a>
<h2>

Ruby library for interacting with the Svix API and verifying webhook signatures

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![Gem](https://img.shields.io/gem/v/svix)](https://rubygems.org/gems/svix)


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
    <th>🚀 None!</th>
  </tr>
</table>

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'svix'
```

And then execute:

```sh
bundle
```

Or install it yourself as:

```sh
gem install svix
```

# Development

First checkout the [core README](../README.md#development) for details on how to generate our API bindings, then follow the steps below.

## Building

```sh
bundle exec rake build
```

## Contributing

Before opening a PR be sure to format your code!

```sh
bundle exec rspec spec
```

## Running Tests

Simply run:

```sh
bundle exec rspec spec
```

## Publishing

```sh
gem build svix.gemspec

gem push pkg/svix-<VERSION>.gem
```
