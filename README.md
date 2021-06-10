<h1 align="center">
  <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
  <p align="center">Svix - Webhooks as a service</p>
</h1>

Libraries for interacting with the Svix API and verifying webhook signatures

![GitHub tag](https://img.shields.io/github/tag/svix/svix-libs.svg)
[![PyPI](https://img.shields.io/pypi/v/svix.svg)](https://pypi.python.org/pypi/svix/)
[![NPM version](https://img.shields.io/npm/v/svix.svg)](https://www.npmjs.com/package/svix)
[![PkgGoDev](https://pkg.go.dev/badge/github.com/svix/svix-libs)](https://pkg.go.dev/github.com/svix/svix-libs)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

# Documentation

The docs are available at <https://docs.svix.com>

# Structure

Each subdirectory has a library for a different language.
The code is a combination of code auto-generated from the OpenAPI spec, and manually written wrappers.

# Building

```sh
# Install deps
yarn
./regen_openapi.sh https://api.svix.com/api/v1/openapi.json
```

Followed by running the build for each of the libraries (subdirectories).
