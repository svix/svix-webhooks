## Start the server (from server/ directory):
cargo run -- --wait-for 15 migrate  # run once to migrate DB
cargo run                            # start the server
The server listens on http://localhost:8071.

## Generate an auth token:
cargo run -- jwt generate

## Use the CLI or curl to test:
Create an application:
curl -X POST http://localhost:8071/api/v1/app/ `
  -H "Authorization: Bearer <TOKEN>" `
  -H "Content-Type: application/json" `
  -d '{"name": "test-app"}'

Create an endpoint (where webhooks are sent to):
curl -X POST http://localhost:8071/api/v1/app/<APP_ID>/endpoint/ `
  -H "Authorization: Bearer <TOKEN>" `
  -H "Content-Type: application/json" `
  -d '{"url": "https://webhook.site/<your-id>", "version": 1}'

Send a message (triggers webhook delivery):
curl -X POST http://localhost:8071/api/v1/app/<APP_ID>/msg/ `
  -H "Authorization: Bearer <TOKEN>" `
  -H "Content-Type: application/json" `
  -d '{"eventType": "test.event", "payload": {"hello": "world"}}'

## For a local webhook receiver, use the CLI's listen command — it creates a public tunnel to your localhost:
svix listen http://localhost:3000/webhook

## Run integration tests (once Postgres auth is resolved):
cargo test --test '*app*'   # test specific area
cargo test                   # all integration tests