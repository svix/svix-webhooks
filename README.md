<p align="center">
  <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
  <h1 align="center">Svix - Webhooks as a service</h1>
</p>

Libraries for interacting with the Svix API and verifying webhook signatures

[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

# Documentation

The docs are available at https://docs.svix.com

# Structure

Each subdirectory has a library for a different language.
The code is a combination of code auto-generated from the OpenAPI spec, and manually written wrappers.

# Building

```
# Install deps
yarn
./regen_openapi.sh https://api.svix.com/api/v1/openapi.json
```

Followed by running the build for each of the libraries (subdirectories).
