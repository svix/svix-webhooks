<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">Svix - Webhooks as a service</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://svix.com">Website</a> | <a href="https://docs.svix.com">Documentation</a> | <a href="https://svix.com/slack">Community Slack</a>
<h2>

Libraries for interacting with the Svix API and verifying webhook signatures

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![PyPI](https://img.shields.io/pypi/v/svix.svg)](https://pypi.python.org/pypi/svix/)
[![NPM version](https://img.shields.io/npm/v/svix.svg)](https://www.npmjs.com/package/svix)
[![Gem](https://img.shields.io/gem/v/svix)](https://rubygems.org/gems/svix)
[![Maven Central (Java)](https://img.shields.io/maven-central/v/com.svix/svix?label=maven-central%20(java))](https://search.maven.org/artifact/com.svix/svix)
[![Maven Central (Kotlin)](https://img.shields.io/maven-central/v/com.svix.kotlin/svix-kotlin?label=maven-central%20(kotlin))](https://search.maven.org/artifact/com.svix.kotlin/svix-kotlin)
[![Nuget](https://img.shields.io/nuget/v/svix)](https://www.nuget.org/packages/Svix/)
[![Packagist Version](https://img.shields.io/packagist/v/svix/svix)](https://packagist.org/packages/svix/svix)
[![PkgGoDev](https://pkg.go.dev/badge/github.com/svix/svix-webhooks)](https://pkg.go.dev/github.com/svix/svix-webhooks/go)

[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

# Usage Documentation

You can find general usage documentation at <https://docs.svix.com>.  For complete API documentation with code examples for each endpoint in all of our official client libraries head over to our API documentation site at <https://api.svix.com>.

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
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/go/README.md">Go</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  </tr>
    <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/python/README.md">Python</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4">Async Support planned.</th>
  </tr>
    </tr>
    <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/javascript/README.md">Typescript/Javascript</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/java/README.md">Java</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4">Async support planned. (If you use kotlin, checkout our kotlin library for coroutine support.)</th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/kotlin/README.md">Kotlin</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4">
  </th>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/ruby/README.md">Ruby</a></th>
    <th>✅</th>
    <th>✅</th>
    <th>✅</th>
    <th colspan="4">
  </th>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/csharp/README.md">C# (dotnet)</a></th>
    <th>✅</th>
    <th>🔜</th>
    <th>✅</th>
    <th colspan="4">
  </th>
  <tr>
    <th><a href="https://github.com/svix/svix-webhooks/blob/main/php/README.md">PHP</a></th>
    <th>✅</th>
    <th>🔜</th>
    <th>✅</th>
    <th colspan="4">
  </th>
</table>

# Development
## Building the client libraries

We use [openapi-generator](https://github.com/OpenAPITools/openapi-generator) to autogenerate bindings in each language. We then use these bindings as the building blocks for our exposed API.  To generate these bindings from the root directory of the project run:

```sh
# Install deps
yarn

# Generate bindings for every language
./regen_openapi.sh
```

Once you have completed this step you can checkout the README of the language of your choice (found in the language's subdirectory) for detailed steps on building that particular library.
