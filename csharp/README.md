# Svix CSharp

## Build

```sh
dotnet build
```

## Format
We use [dotnet-format](https://github.com/dotnet/format) for this project.

First install it then run:
```sh
dotnet-format
```

## Test

```sh
dotnet test Svix.Tests
```

## Publish

```sh
dotnet nuget push path/to/Svix.X.X.X.nupkg --api-key XXXXX --source https://api.nuget.org/v3/index.json
```