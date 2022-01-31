<h1 align="center">
  <a href="https://www.svix.com">
    <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
    <p align="center">Svix - Webhooks as a service</p>
  </a>
</h1>

Libraries for interacting with the Svix API and verifying webhook signatures

![GitHub tag](https://img.shields.io/github/tag/svix/svix-libs.svg)
[![PyPI](https://img.shields.io/pypi/v/svix.svg)](https://pypi.python.org/pypi/svix/)
[![NPM version](https://img.shields.io/npm/v/svix.svg)](https://www.npmjs.com/package/svix)
[![Gem](https://img.shields.io/gem/v/svix)](https://rubygems.org/gems/svix)
[![Maven Central (Java)](https://img.shields.io/maven-central/v/com.svix/svix?label=maven-central%20(java))](https://search.maven.org/artifact/com.svix/svix)
[![Maven Central (Kotlin)](https://img.shields.io/maven-central/v/com.svix.kotlin/svix-kotlin?label=maven-central%20(kotlin))](https://search.maven.org/artifact/com.svix.kotlin/svix-kotlin)
[![Nuget](https://img.shields.io/nuget/v/svix)](https://www.nuget.org/packages/Svix/)
[![Packagist Version](https://img.shields.io/packagist/v/svix/svix)](https://packagist.org/packages/svix/svix)
[![PkgGoDev](https://pkg.go.dev/badge/github.com/svix/svix-libs)](https://pkg.go.dev/github.com/svix/svix-libs/go)

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
