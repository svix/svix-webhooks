module github.com/svix/svix-webhooks

go 1.20

retract (
	v1.46.0
	v1.47.0
	v1.48.0
	v1.49.0
	v1.50.0
	v1.51.0
	v1.52.0
	// v1.53.0 was never published
	v1.54.0
	v1.55.0
)

require (
	golang.org/x/oauth2 v0.0.0-20210514164344-f6687ab2804c
	gopkg.in/validator.v2 v2.0.1
)

require (
	github.com/golang/protobuf v1.5.0 // indirect
	golang.org/x/net v0.23.0 // indirect
	google.golang.org/appengine v1.6.6 // indirect
	google.golang.org/protobuf v1.33.0 // indirect
)
