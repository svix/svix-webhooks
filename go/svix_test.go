package svix_test

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/url"
	"os"
	"testing"

	svix "github.com/svix/svix-webhooks/go"
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

func TestStaticNullableString(t *testing.T) {
	app := &svix.ApplicationPatch{
		Uid: svix.StaticNullableString("my-uid"),
	}

	if !app.Uid.IsSet() {
		t.Fatalf("app.Uid is not set but should be")
	}

	if *app.Uid.Get() != "my-uid" {
		t.Fatalf("app.Uid has unexpected value: `%s`", *app.Uid.Get())
	}

	app.Uid.Unset()
	if app.Uid.IsSet() {
		t.Fatalf("app.Uid is set but shouldn't be")
	}
}

func assertMarshalEq(v any, expected string, t *testing.T) {
	bytes, err := json.Marshal(v)
	if err != nil {
		t.Error("JSON serialization failed", err)
		return
	}

	s := string(bytes)
	if s != expected {
		t.Error("Unexpected serialization", s)
	}
}

func TestModelSerialization(t *testing.T) {
	ep_in := svix.EndpointIn{
		Url: "http://example.local",
	}
	assertMarshalEq(ep_in, `{"url":"http://example.local"}`, t)

	uid := "test"
	ep_in.Uid = &uid
	assertMarshalEq(ep_in, `{"uid":"test","url":"http://example.local"}`, t)

	uid = ""
	ep_in.Uid = &uid
	assertMarshalEq(ep_in, `{"uid":"","url":"http://example.local"}`, t)

	metadata := make(map[string]string)
	ep_in.Uid = nil
	ep_in.Metadata = &metadata
	assertMarshalEq(ep_in, `{"metadata":{},"url":"http://example.local"}`, t)

	ep_patch := svix.EndpointPatch{}
	assertMarshalEq(ep_patch, `{}`, t)

	// FIXME: setting channels to `null` not currently possible
	ep_patch.Channels = []string{}
	assertMarshalEq(ep_patch, `{"channels":[]}`, t)

	ep_patch.Channels = []string{"foo"}
	assertMarshalEq(ep_patch, `{"channels":["foo"]}`, t)

	ep_patch.Channels = nil
	disabled := false
	ep_patch.Disabled = &disabled
	assertMarshalEq(ep_patch, `{"disabled":false}`, t)

	ep_patch.Disabled = nil
	ep_patch.Uid = *svix.NullableString(nil)
	assertMarshalEq(ep_patch, `{"uid":null}`, t)
}

func TestModelDeserialization(t *testing.T) {
	var ep_out svix.EndpointOut

	// only required properties
	err := json.Unmarshal(
		[]byte(`{
			"id": "ep_foo",
			"metadata": {"foo": "bar"},
			"description": "x",
			"url": "http://example.local",
			"version": 1,
			"createdAt": "2019-08-24T14:15:22Z",
			"updatedAt": "2019-08-24T14:15:22Z"
		}`),
		&ep_out,
	)
	if err != nil {
		t.Fatal("JSON deserialization failed", err)
	}
	if len(ep_out.Channels) != 0 {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
	if ep_out.Id != "ep_foo" {
		t.Error("unexpected value for id", ep_out.Id)
	}
	if len(ep_out.Metadata) != 1 {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Metadata["foo"] != "bar" {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Description != "x" {
		t.Error("unexpected value for description", ep_out.Description)
	}
	if ep_out.Disabled != nil {
		t.Error("unexpected value for disabled", ep_out.Disabled)
	}
	if len(ep_out.FilterTypes) != 0 {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if ep_out.RateLimit != nil {
		t.Error("unexpected value for rateLimit", ep_out.RateLimit)
	}
	if ep_out.Uid != nil {
		t.Error("unexpected value for uid", ep_out.Uid)
	}
	if ep_out.Url != "http://example.local" {
		t.Error("unexpected value for url", ep_out.Url)
	}
	if ep_out.Version != 1 {
		t.Error("unexpected value for version", ep_out.Version)
	}

	// full example from API docs
	err = json.Unmarshal(
		[]byte(`{
			"id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
			"metadata": {
				"property1": "string",
				"property2": "string"
			},
			"description": "string",
			"rateLimit": 0,
			"uid": "unique-ep-identifier",
			"url": "https://example.com/webhook/",
			"version": 1,
			"disabled": false,
			"filterTypes": [
				"user.signup",
				"user.deleted"
			],
			"channels": [
				"project_123",
				"group_2"
			],
			"createdAt": "2019-08-24T14:15:22Z",
			"updatedAt": "2019-08-24T14:15:22Z"
		}`),
		&ep_out,
	)
	if err != nil {
		t.Fatal("JSON deserialization failed", err)
	}
	if ep_out.Id != "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2" {
		t.Error("unexpected value for id", ep_out.Id)
	}
	if len(ep_out.Metadata) != 2 {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Metadata["property1"] != "string" {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Metadata["property2"] != "string" {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Description != "string" {
		t.Error("unexpected value for description", ep_out.Description)
	}
	if *ep_out.RateLimit != 0 {
		t.Error("unexpected value for rateLimit", ep_out.RateLimit)
	}
	if *ep_out.Uid != "unique-ep-identifier" {
		t.Error("unexpected value for uid", ep_out.Uid)
	}
	if ep_out.Url != "https://example.com/webhook/" {
		t.Error("unexpected value for url", ep_out.Url)
	}
	if ep_out.Version != 1 {
		t.Error("unexpected value for version", ep_out.Version)
	}
	if *ep_out.Disabled != false {
		t.Error("unexpected value for disabled", ep_out.Disabled)
	}
	if len(ep_out.FilterTypes) != 2 {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if ep_out.FilterTypes[0] != "user.signup" {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if ep_out.FilterTypes[1] != "user.deleted" {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if len(ep_out.Channels) != 2 {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
	if ep_out.Channels[0] != "project_123" {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
	if ep_out.Channels[1] != "group_2" {
		t.Error("unexpected value for channels", ep_out.Channels)
	}

	// same example with optional / nullable fields set to null explicitly
	// (and empty metadata)
	err = json.Unmarshal(
		[]byte(`{
			"id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
			"metadata": {},
			"description": "string",
			"rateLimit": null,
			"uid": null,
			"url": "https://example.com/webhook/",
			"version": 1,
			"disabled": null,
			"filterTypes": null,
			"channels": null,
			"createdAt": "2019-08-24T14:15:22Z",
			"updatedAt": "2019-08-24T14:15:22Z"
		}`),
		&ep_out,
	)
	if err != nil {
		t.Fatal("JSON deserialization failed", err)
	}
	if ep_out.Id != "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2" {
		t.Error("unexpected value for id", ep_out.Id)
	}
	if len(ep_out.Metadata) != 0 {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Description != "string" {
		t.Error("unexpected value for description", ep_out.Description)
	}
	if ep_out.RateLimit != nil {
		t.Error("unexpected value for rateLimit", ep_out.RateLimit)
	}
	if ep_out.Uid != nil {
		t.Error("unexpected value for uid", ep_out.Uid)
	}
	if ep_out.Url != "https://example.com/webhook/" {
		t.Error("unexpected value for url", ep_out.Url)
	}
	if ep_out.Version != 1 {
		t.Error("unexpected value for version", ep_out.Version)
	}
	if ep_out.Disabled != nil {
		t.Error("unexpected value for disabled", ep_out.Disabled)
	}
	if len(ep_out.FilterTypes) != 0 {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if len(ep_out.Channels) != 0 {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
}
