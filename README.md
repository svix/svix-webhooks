<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">Svix - Webhooks as a service</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://svix.com">Website</a> | <a href="https://docs.svix.com">Documentation</a> | <a href="https://svix.com/slack">Community Slack</a>
<h2>

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![Build Status](https://github.com/svix/svix-webhooks/workflows/Server%20CI/badge.svg)](https://github.com/svix/svix-webhooks/actions)
[![Server Security](https://github.com/svix/svix-webhooks/actions/workflows/server-security.yml/badge.svg)](https://github.com/svix/svix-webhooks/actions/workflows/server-security.yml)
[![Twitter Follow](https://img.shields.io/twitter/follow/SvixHQ?style=social)](https://twitter.com/SvixHQ)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

## Svix is the enterprise ready webhook service

Svix makes it easy for developers to send webhooks. Developers make one API call, and Svix takes care of deliverability, retries, security, and more. For more information, please refer to the [Svix homepage](https://www.svix.com).


[![PyPI](https://img.shields.io/pypi/v/svix.svg)](https://pypi.python.org/pypi/svix/)
[![Crates.io](https://img.shields.io/crates/v/svix)](https://crates.io/crates/svix)
[![NPM version](https://img.shields.io/npm/v/svix.svg)](https://www.npmjs.com/package/svix)
[![Gem](https://img.shields.io/gem/v/svix)](https://rubygems.org/gems/svix)
[![Maven Central (Java)](https://img.shields.io/maven-central/v/com.svix/svix?label=maven-central%20(java))](https://search.maven.org/artifact/com.svix/svix)
[![Maven Central (Kotlin)](https://img.shields.io/maven-central/v/com.svix.kotlin/svix-kotlin?label=maven-central%20(kotlin))](https://search.maven.org/artifact/com.svix.kotlin/svix-kotlin)
[![Nuget](https://img.shields.io/nuget/v/svix)](https://www.nuget.org/packages/Svix/)
[![Packagist Version](https://img.shields.io/packagist/v/svix/svix)](https://packagist.org/packages/svix/svix)
[![PkgGoDev](https://pkg.go.dev/badge/github.com/svix/svix-webhooks)](https://pkg.go.dev/github.com/svix/svix-webhooks/go)

# Documentation

You can find general usage documentation at <https://docs.svix.com>. For complete API documentation with code examples for each endpoint in all of our official client libraries head over to our API documentation site at <https://api.svix.com>.

# Support & Community

  - [GitHub Issues](https://github.com/svix/svix-webhooks/issues) - report issues and make suggestions.
  - [Community Forum](https://github.com/svix/svix-webhooks/discussions) - ask questions, and start discussions!
  - [Slack](https://www.svix.com/slack/) - come and chat with us!

To stay up-to-date with new features and improvements be sure to watch our repo!

![Watch & Star our repo](/static-assets/watch.gif)

# Client Library Overview

<table style="table-layout:fixed; white-space: nowrap;">
  <th colspan="6">⚡️ Feature Breakdown ⚡️</th>
  <tr>
    <th>Language</th>
    <th>Officially Supported</th>
    <th>API Support</th>
    <th>Webhook Verification</th>
    <th colspan="4">Other Notes</th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/go/">Go</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  </tr>
    <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/python/">Python</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
    </tr>
    <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/javascript/">Typescript/Javascript</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/java/">Java</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4">Async support planned. (If you use kotlin, checkout our kotlin library for coroutine support.)</th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/kotlin/">Kotlin</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/ruby/">Ruby</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/csharp/">C# (dotnet)</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/rust/">Rust</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/tree/main/php/">PHP</a></th>
    <th>✅</th>
    <th>🔜</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
</table>

# Running the server

There are multiple ways to get the Svix server up running. Docker is probably the most common one, but you can choose the one that works best for you.

The Svix server is written in Rust 🦀, which means you can compile it into a static library for a variety of targets. Please refer to the building from source section below for more information.

Please refer to the [server configuration](#server-configuration) section below for more information regarding the available settings.

## Deployment

### Docker

You can use the official Svix Docker image from [Docker Hub](https://hub.docker.com/r/svix/svix-server). You can either use the `latest` tag, or one of [the versioned tags](https://hub.docker.com/r/svix/svix-server/tags) instead.

You can either use the example [docker-compose.yml](./server/docker-compose.yml) file with `docker-compose` (easiest), `docker swarm` (advanced), or run the container standalone.

#### With Docker Compose

This alternative is the easiest because it will also boot up and configure `redis` and `postgresql`.

This assumes you have docker-compose installed.

```
cd server
docker-compose up
```

#### Standalone container

Running a standalone container is slightly more advanced, as it requires you to set some environment variables and have them pointing to your `redis` and `postgres` instances.
You can pass individual environment variables to docker using the `-e` flag, or just create a file like [development.env](./server/svix-server/development.env) and use the `--env-file` flag like in the example below:

```
docker run \
  --name svix-server \
  -p 8071:8071 \
  --env-file development.env \
  svix/svix-server
```

### Pre-compiled binaries

Pre-compiled binaries are available for released versions in the [releases section](https://github.com/svix/svix-webhooks/releases).

### Building from source

The Svix server is written in Rust 🦀 and requires a Rust build environment.

If you already have one, you just need to run `cargo build`, otherwise, please please refer to the [Svix server README](./server/#readme) for more information about building the server from source.

## Runtime dependencies

The server requires the following runtime dependencies to work correctly:

- A PostgreSQL server - for the storage of events.
- An *optional* Redis server version 6.2.0 or higher - for the task queue and cache. Please note that it's recommended to enable persistence in Redis so that tasks are persisted across Redis server restarts and upgrades.

## Server configuration

There are three ways to configure `svix-server`: environment vars, `.env` file, and a configuration file.

### Configuration file

You can put a file called `config.toml` in the current working directory of `svix-server` and it will automatically pick it up.
You can take a look at the example file for more information and a full list of supported settings: [config.toml](./server/svix-server/config.default.toml).

Here's a quick example of the most important configurations:

```toml
# The JWT secret for authentication - should be secret and securely generated
jwt_secret = "8KjzRXrKkd9YFcNyqLSIY8JwiaCeRc6WK4UkMnSW"

# The DSN for the database. Only postgres is currently supported.
db_dsn = "postgresql://postgres:postgres@pgbouncer/postgres"

# The DSN for redis (can be left empty if not using redis)
redis_dsn = "redis://redis:6379"

# What kind of message queue to use.
queue_type = "redis"
```


### Environment (variables or `.env`)

Alternatively, you can configure `svix-server` by setting the equivalent environment variables for each of the supported settings. The environment variables can either be passed directly or by setting them in a `.env` file.

The environment variables have the name name as the config names, but they are all upper case and are prefixed with `SVIX_`.

For example, the above example configuration would look like this if it was passed in the env:

```
# The JWT secret for authentication - should be secret and securely generated
SVIX_JWT_SECRET = "8KjzRXrKkd9YFcNyqLSIY8JwiaCeRc6WK4UkMnSW"

# The DSN for the database. Only postgres is currently supported.
SVIX_DB_DSN = "postgresql://postgres:postgres@pgbouncer/postgres"

# The DSN for redis (can be left empty if not using redis)
SVIX_REDIS_DSN = "redis://redis:6379"

# What kind of message queue to use.
SVIX_QUEUE_TYPE = "redis"
```

### OpenTelemetry

You may send tracing information to the OpenTelemetry Collector which allows forwarding trace events to a number of external applications/services such as DataDog, Jaeger, NewRelic, Prometheus, Sentry, Signoz, and Zipkin.

You can see more in [these instructions](./OpenTelemetry.md).

### Connection Pool Size

There are two configuration variables `db_pool_max_size` and `redis_pool_max_size` which control the maximum allowed size of the connection pool for PostgreSQL and Redis respectively.

They default to a max size of 20, but higher values can significantly increase performance if your database can handle it.

### SSRF Attacks and Internal IP Addresses
To prevent SSRF attacks, message dispatches to internal IP addresses are blocked by default. However we understand that this doesn't meet the needs of every user say, for example, the service can only be accessed internally. To bypass these restrictions, see the `whitelist_subnets` configuration option, which accepts an array of CIDR-notation subnets to allow messages to be dispatched to.

### Webhook signature scheme (symmetric vs asymmetric)

To ensure the security and integrity of messages, Svix signs all webhook messages prior to sending.
Svix supports two types of signature schemes: symmetric (pre-shared key) and asymmetric (public key).

Symmetric signatures are significantly faster (~50x for signing, and ~160x for verifying), and are much simpler (which makes verification easier for your customers), though they require the usage of a pre-shared key per endpoint (endpoint secret) in order to work. Asymmetric signatures on the other hand only require sharing a public key with your customers (not secret).

Because of the above, using symmetric keys is both recommended and the Svix default. Using them is documented in the [verifying signatures section of the docs](https://docs.svix.com/receiving/verifying-payloads/how-manual).

However, in some scenarios it may be beneficial to use asymmetric signatures, which is why they too are supported. For more information please refer to the [asymmetric signatures section](#asymmetric-signatures) below.

## Authentication

Use valid JWTs generated with the correct secret as `Bearer`.

E.g:
```
Authorization: Bearer <JWT_TOKEN_HERE>
```

Either generate one using
```
svix-server jwt generate
```

Or if you are generating your own, make sure to use `org_23rb8YdGqMT0qIzpgGwdXfHirMu` as the `sub` field, and `H256` as the algorithm.


Example valid JWT for the secret `x` (so you can see the structure):
```js
// JWT: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE2NTUxNDA2MzksImV4cCI6MTk3MDUwMDYzOSwibmJmIjoxNjU1MTQwNjM5LCJpc3MiOiJzdml4LXNlcnZlciIsInN1YiI6Im9yZ18yM3JiOFlkR3FNVDBxSXpwZ0d3ZFhmSGlyTXUifQ.USMuIPrqsZTSj3kyWupCzJO9eyQioBzh5alGlvRbrbA
// Structure (when decoded):
{
  "iat": 1655140639,
  "exp": 1970500639,
  "nbf": 1655140639,
  "iss": "svix-server",
  "sub": "org_23rb8YdGqMT0qIzpgGwdXfHirMu"
}
```

## Operational (incoming) webhooks

Operational webhooks are webhooks that you can subscribe to in order to get notified of important events occurring on the svix-server. The list of supported events is available in [the webhooks section of the API reference](https://api.svix.com/docs#tag/Webhooks).

The operational webhooks utilize Svix, and are controlled by a special account with the following ID: `org_00000000000SvixManagement00`.
To turn operational webhooks on, set the `operational_webhook_address` config to point to your Svix server, and create a JWT for the special account.
Once those are set, create an `Application` with the `uid` set to the `org_id` you're interested in, and add `Endpoint`s for all of the events you'd like to subscribe to.

For example, for the default account, just create an app with the `uid` set to `org_23rb8YdGqMT0qIzpgGwdXfHirMu`.

## Asymmetric signatures

As mentioned above, symmetric signatures are recommended. However, please read the following instructions on setting up asymmetric signatures if you have determined that asymmetric signatures are what you need.

### Configuring the keys

By default, the Svix server generates symmetric secrets for endpoints, which in turn means messages will be signed with symmetric keys. To change this default, set the `default_signature_type` config to `ed25519` as follows:

```toml
default_signature_type = "ed25519"
```

Additionally, no matter what the default is set to, you can still override it by explicitly setting a key on an endpoint.
To set a symmetric key, set the endpoint secret to a secret prefixed with `whsec_`, such as `whsec_51TKyHBy5KFY1Ab98GQ8V60BkWnejkWy`.
To set an asymmetric key, set the endpoint secret to a **valid** ed25519 base64 encoded private key prefixed with `whsk_` such as: `whsk_6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==`.

Please note, that the expected private key structure is: `whsk_${base64(private_key + public_key)}`.

For testing purposes, new asymmetric key pairs can be generated using the following command:
```bash
$ svix-server asymmetric-key generate

Secret key: whsk_6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==
Public key: whpk_1SiA4o9hyqTCpIqC5V9HUakiiaeACeqfZTInDBbOir4=
```

### Signature scheme

Svix uses `ed25519(m)` for signing the webhook messages, and it constructs `m` the same way as it does for the symmetric signatures.

When verifying the message you should also ensure that the timestamp is recent enough in order to limit the potential of replay attacks as noted in [the symmetric verification docs](https://docs.svix.com/receiving/verifying-payloads/why).

## Shutting down the server

To support graceful shutdown on the server, all running tasks are finished before shutting down on a SIGINT/SIGTERM. This usually takes less than ten seconds.

# Differences to the Svix hosted service

One of our main goals with open sourcing the Svix dispatcher is ease of use. The hosted Svix service, however, is quite complex due to our scale and the infrastructure it requires. This complexity is not useful for the vast majority of people and would make this project much harder to use and much more limited.
This is why this code has been adjusted before being released, and some of the features, optimizations, and behaviors supported by the hosted dispatcher are not yet available in this repo. With that being said, other than some known incompatibilities, the internal Svix test suite passes. This means they are already mostly compatible, and we are working hard on bringing them to full feature parity.


# Development

Checkout our project specific development guides to get started hacking on Svix!

- [Svix libs README](./README_LIBS.md)
- [Svix server README](./server/#readme)

# Contributing

Contributions are what makes the open source world go round! All contributions are very much welcomed and are greatly appreciated.

Please refer to the [contribution guide](./CONTRIBUTING.md) for information on how to contribute.

A quick how to for contribution:
    
1. Fork the project
2. Create your feature branch (`git checkout -b feature/some-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Implement an amazing feature.'`)
5. Push to the branch (`git push origin feature/some-feature`)
6. Open a pull request


# License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

# Backed By

![Backed By YC & Aleph](/static-assets/backed-by.png)
