module github.com/svix/svix-webhooks

go 1.20

retract (
	// versions with a bug that silently broke downstream error handling code
	v1.61.0
	v1.60.1
	v1.60.0
	v1.59.2
	v1.59.1
	v1.59.0
	v1.58.2
	v1.58.1
	v1.58.0
	v1.57.0

	// accidentally-published test versions
	v1.55.0
	v1.54.0
	// v1.53.0 was never published
	v1.52.0
	v1.51.0
	v1.50.0
	v1.49.0
	v1.48.0
	v1.47.0
	v1.46.0
)

require gopkg.in/validator.v2 v2.0.1

require github.com/jarcoal/httpmock v1.3.1 // indirect
