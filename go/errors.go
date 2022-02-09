package svix

import (
	"net/http"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

// Error provides access to the body, status, and error on returned errors.
type Error struct {
	status int
	body   []byte
	error  string
}

// Error returns non-empty string if there was an error.
func (e Error) Error() string {
	return e.error
}

// Body returns the raw bytes of the response.
func (e Error) Body() []byte {
	return e.body
}

// Status returns the HTTP status of the error.
func (e Error) Status() int {
	return e.status
}

// a simple function convert openapi errors to exposed svix.Error
func wrapError(err error, res *http.Response) error {
	if openapiError, ok := err.(openapi.GenericOpenAPIError); ok {
		e := &Error{
			body:  openapiError.Body(),
			error: openapiError.Error(),
		}
		if res != nil {
			e.status = res.StatusCode
		}
		return e
	}
	return err
}
