<h1 align="center">
  <a href="https://www.svix.com">
    <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
    <p align="center">Svix - Webhooks as a service</p>
  </a>
</h1>

# Svix CLI

A CLI to interact with the Svix API.

**With the Svix CLI, you can:**

- Interact with the Svix API
- Validate Webhook payloads


## Installation

### Pre-built executables

#### Via installer scripts

Pre-built binaries are available for Linux, macOS via shell script installers.

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/svix/svix-webhooks/releases/download/v1.75.0/svix-cli-installer.sh | sh
```

For Windows users, installation can be done via powershell:

```
powershell -ExecutionPolicy ByPass -c "irm https://github.com/svix-onelson/svix-webhooks/releases/download/v1.75.0/svix-cli-installer.ps1 | iex"
```

These scripts will install the binaries to `~/.svix/bin` and also add this directory to your `PATH` by default.

#### Manually

Additionally, you can select executables to download directly from [our releases page](https://github.com/svix/svix-webhooks/releases), and use them as is without
having to install anything.

1. Download and extract the archive for your operating system.
2. Run the `svix` executable from the command line: `./svix help`.

> [!NOTE]
> You may need to allow execution by running `chmod +x svix`.


You can also put the binaries anywhere in your `PATH` so you can run the command from anywhere without needing to provide its full path. On macOS or Linux you can achieve this by moving the executable to `/usr/local/bin` or `/usr/bin`.


## Usage

Installing the Svix CLI provides access to the `svix` command.

```sh
svix [command]

# Run `svix help` for information about the available commands
svix help

# or add the `--help` flag to any command for a more detailed description and list of flags
svix [command] --help
```


## Using the `listen` command

The `listen` command creates an on-the-fly publicly accessible URL for use when testing webhooks.

> [!NOTE]
> You don't need a Svix account when using the `listen` command.

The cli then acts as a proxy, forwarding any requests to the given local URL.
This is useful for testing your webhook server locally without having to open a port or
change any NAT configuration on your network.

Example:

```sh
svix listen http://localhost:8000/webhook/
```

Output:

```sh
Webhook Relay is now listening at:
https://play.svix.com/in/c_tSdQhb4Q5PTF5m2juiWu8qFREqE/

All requests on this endpoint will be forwarded to your local URL:
http://localhost:8080/webhook/

View logs and debug information at:
https://play.svix.com/view/c_tSdQhb4Q5PTF5m2juiWu8qFREqE/
```

The above command will return you a unique URL and forward any POST requests it receives
to `http://localhost:8000/webhook/`.

## Interacting with the Svix Server

```sh
# Set your Auth Token temporarily via the SVIX_AUTH_TOKEN environment variable
export SVIX_AUTH_TOKEN=<MY-AUTH-TOKEN>
# or to persistently store your auth token in a config file run
svix login # interactively configure your Svix API credentials

# Create an Application with the name "Demo"
svix application create '{ "name": "demo" }'

# List Applications
svix application list --limit 2 --iterator some_iterator
```

## Commands

The Svix CLI supports the following commands:
| Command         | Description                                                |
| --------------- | ---------------------------------------------------------- |
| login           | Interactively configure your Svix API credentials          |
| application     | List, create & modify applications                         |
| authentication  | Manage authentication tasks such as getting dashboard URLs |
| endpoint        | List, create & modify endpoints                            |
| event-type      | List, create & modify event types                          |
| message         | List & create messages                                     |
| message-attempt | List, lookup & resend message attempts                     |
| signature       | Sign or Verify the signature of a webhook message          |
| listen          | Forward webhook requests a local url                       |
| integration     | List, create & modify integrations                         |
| open            | Quickly open Svix pages in your browser                    |
| completion      | Generate completion script                                 |
| version         | Get the version of the Svix CLI                            |
| help            | Help about any command                                     |


## Shell Completions

Shell completion scripts are provided for `bash`, `elvish`, `fish`, `powershell`, and `zsh`.

To generate a script for your shell type `svix completion <SHELL NAME>`.

For detailed instructions on configuring completions for your shell run `svix completion --help`.

> [!TIP]
> You can source the completion script automatically in your shell rc file.
>
> Example:
> ```sh
> eval "$(svix completion bash)"
> ```


## Documentation

For a more information, checkout our [API reference](https://docs.svix.com).
