# Svix Java

Please refer to [the documentation](https://docs.svix.com) for usage instructions.

## Requirements

Building the API client library requires:
1. Java 1.8+
2. Gradle

## Installation

### Maven users

Add this dependency to your project's POM:

```xml
<dependency>
  <groupId>com.svix</groupId>
  <artifactId>svix</artifactId>
  <version>0.30.0</version>
  <scope>compile</scope>
</dependency>
```

### Gradle users

Add this dependency to your project's build file:

```groovy
implementation "com.svix:svix:0.30.0"
```


## Development

### Build

./gradlew build

### Test

./gradlew test

### Publish

./gradlew uploadArchives
