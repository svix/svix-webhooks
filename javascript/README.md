<h1 align="center">
  <a href="https://www.svix.com">
    <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
    <p align="center">Svix - Webhooks as a service</p>
  </a>
</h1>

Library for interacting with the Svix API and verifying webhook signatures

[![NPM version](https://img.shields.io/npm/v/svix.svg)](https://www.npmjs.com/package/svix)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

## 📕 Documentation

The docs are available at <https://docs.svix.com>.


## ⚙️ Installation

```sh
npm install svix
# or
yarn add svix
```

## ⚡️ Usage

```js
import { Svix } from "svix";

const svix = new Svix("AUTH_TOKEN");
const app = await svix.application.create({ name: "Application name" });
```