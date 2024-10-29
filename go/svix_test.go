package svix_test

import (
	"context"
	"errors"
	svix "github.com/svix/svix-webhooks/go"
	"net/http"
	"net/url"
	"os"
	"testing"
)

// Builds an API client for testing against an arbitrary API server with an arbitrary token.
//
// The connection details are pulled from the environment, e.g. `SVIX_TOKEN` and `SVIX_SERVER_URL`.
// In the case that either are unset, a test that calls this function will automatically skip.
func getTestClient(t *testing.T) *svix.Svix {
	t.Helper()

	token, exists := os.LookupEnv("SVIX_TOKEN")
	if !exists {
		t.Skipf("Unable to construct test client (`SVIX_TOKEN` unset)")
		return nil
	}
	rawServerUrl, exists := os.LookupEnv("SVIX_SERVER_URL")
	if !exists {
		t.Skipf("Unable to construct test client (`SVIX_SERVER_URL` unset)")
		return nil
	}
	serverUrl, err := url.Parse(rawServerUrl)
	if err != nil {
		panic(err)
	}
	return svix.New(token, &svix.SvixOptions{
		ServerUrl: serverUrl,
	})
}

// Suppresses a request error response if it's a 409
func isNotConflict(err error) error {
	if err != nil {
		var svixError *svix.Error
		if errors.As(err, &svixError) {
			if svixError.Status() == http.StatusConflict {
				// Pass if we see the suppressed status
				return nil
			}
		}
	}
	return err
}

// Runs through some common API interactions.
func TestKitchenSink(t *testing.T) {
	ctx := context.Background()
	client := getTestClient(t)

	app, err := client.Application.Create(ctx, &svix.ApplicationIn{
		Name: "test",
	})
	if err != nil {
		t.Fatal(err)
	}

	_, err = client.EventType.Create(ctx, &svix.EventTypeIn{Name: "event.started", Description: "Something started"})

	if isNotConflict(err) != nil {
		t.Fatal(err)
	}

	_, err = client.EventType.Create(ctx, &svix.EventTypeIn{Name: "event.ended", Description: "Something ended"})
	if isNotConflict(err) != nil {
		t.Fatal(err)
	}

	endp, err := client.Endpoint.Create(ctx, app.Id, &svix.EndpointIn{
		Url:      "https://example.svix.com/",
		Channels: []string{"ch0", "ch1"},
	})
	if err != nil {
		t.Fatal(err)
	}

	if len(endp.GetChannels()) != 2 {
		t.Errorf("got %d channels, want 2", len(endp.Channels))
	}
	if len(endp.GetFilterTypes()) != 0 {
		t.Errorf("got %d filter types, want 0", len(endp.GetFilterTypes()))
	}
	endpPatch := svix.EndpointPatch{}
	endpPatch.SetFilterTypes([]string{"event.started", "event.ended"})

	patched, err := client.Endpoint.Patch(ctx, app.Id, endp.Id, &endpPatch)
	if err != nil {
		t.Fatal(err)
	}

	for _, typ := range patched.GetFilterTypes() {
		if !(typ == "event.started" || typ == "event.ended") {
			t.Fatalf("unexpected filter type: `%s`", typ)
		}
	}

	endpPatch2 := svix.EndpointPatch{
		Channels: nil,
	}
	patched2, err := client.Endpoint.Patch(ctx, app.Id, endp.Id, &endpPatch2)
	if err != nil {
		t.Fatal(err)
	}
	if len(patched2.Channels) > 0 {
		t.Fatalf("expected patched channels to be empty: %v", patched2.Channels)
	}

	err = client.Endpoint.Delete(ctx, app.Id, endp.Id)
	if err != nil {
		t.Fatalf("unexpected error on delete: %s", err.Error())
	}
}
