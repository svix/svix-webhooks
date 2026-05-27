<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">Svix - Webhooks as a service</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://svix.com">Website</a> | <a href="https://docs.svix.com">Documentation</a> | <a href="https://svix.com/slack">Community Slack</a>
<h2>

Python library for interacting with the Svix API and verifying webhook signatures

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![PyPI](https://img.shields.io/pypi/v/svix.svg)](https://pypi.python.org/pypi/svix/)

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

# Installation

```sh
pip install svix
```

## Usage
Please refer to [the documentation](https://docs.svix.com/) or [the API reference](https://api.svix.com/docs) for more usage instructions.

### Async
```python
from svix.api import SvixAsync, ApplicationIn

svix = SvixAsync("AUTH_TOKEN")
app = await svix.application.create(ApplicationIn(name="Application name"))
```

### Sync

```python
from svix.api import Svix, ApplicationIn

svix = Svix("AUTH_TOKEN")
app = svix.application.create(ApplicationIn(name="Application name"))
```

# Development

First checkout the [core README](../README.md#development) for details on how to generate our API bindings, then follow the steps below.

## Requirements

 - [CPython](https://www.python.org/) 3.8+
 - [uv](https://github.com/astral-sh/uv)

## Installing dependencies

```sh
uv sync --frozen --all-groups
```

## Contributing

Before opening a PR be sure to format your code!

```sh
uv run ./scripts/format.sh
uv run ./scripts/lint.sh
```

## Running Tests

Simply run:

```sh
uv run pytest
```

Note that this will run integration tests that require a running Docker/Podman/etc server and working [`docker-compose`](https://docs.docker.com/compose/)
