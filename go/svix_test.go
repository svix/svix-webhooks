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
	"github.com/svix/svix-webhooks/go/models"
	"github.com/svix/svix-webhooks/go/utils"
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
	svx, err := svix.New(token, &svix.SvixOptions{
		ServerUrl: serverUrl,
	})
	if err != nil {
		panic(err)
	}
	return svx
}

// Suppresses a request error response if it's a 409
func isNotConflict(err error) error {
	if err != nil {
		var svixError svix.Error
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

	app, err := client.Application.Create(ctx, &models.ApplicationIn{
		Name: "test",
	}, nil)
	if err != nil {
		t.Fatal(err)
	}

	_, err = client.EventType.Create(ctx, &models.EventTypeIn{Name: "event.started", Description: "Something started"}, nil)

	if isNotConflict(err) != nil {
		t.Fatal(err)
	}

	_, err = client.EventType.Create(ctx, &models.EventTypeIn{Name: "event.ended", Description: "Something ended"}, nil)
	if isNotConflict(err) != nil {
		t.Fatal(err)
	}

	endp, err := client.Endpoint.Create(ctx, app.Id, &models.EndpointIn{
		Url: "https://example.svix.com/",
	}, nil)
	if err != nil {
		t.Fatal(err)
	}

	endpPatch := models.EndpointPatch{
		FilterTypes: utils.NewNullable([]string{"event.started", "event.ended"}),
	}

	patched, err := client.Endpoint.Patch(ctx, app.Id, endp.Id, &endpPatch)
	if err != nil {
		t.Fatal(err)
	}

	for _, typ := range patched.FilterTypes {
		if !(typ == "event.started" || typ == "event.ended") {
			t.Fatalf("unexpected filter type: `%s`", typ)
		}
	}
}

func TestStaticNullableString(t *testing.T) {
	app := &models.ApplicationPatch{
		Uid: utils.NewNullable("my-uid"),
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
		t.Errorf("Unexpected serialization expected: `%s`  got: `%s`", expected, s)
	}
}

func TestModelSerialization(t *testing.T) {
	ep_in := models.EndpointIn{
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

	ep_patch := models.EndpointPatch{}
	assertMarshalEq(ep_patch, `{}`, t)

	// FIXME: setting channels to `null` not currently possible
	ep_patch.Channels = utils.NewNullable([]string{})
	assertMarshalEq(ep_patch, `{"channels":[]}`, t)

	ep_patch.Channels = utils.NewNullable([]string{"foo"})
	assertMarshalEq(ep_patch, `{"channels":["foo"]}`, t)

	ep_patch.Channels = utils.NewNilUnsetNullable[[]string]()
	disabled := false
	ep_patch.Disabled = &disabled
	assertMarshalEq(ep_patch, `{"disabled":false}`, t)

	ep_patch.Disabled = nil
	ep_patch.Uid = utils.NewNilNullable[string]()
	assertMarshalEq(ep_patch, `{"uid":null}`, t)
}

func TestModelDeserialization(t *testing.T) {
	var ep_out models.EndpointOut

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

	ep_out = models.EndpointOut{}
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

	ep_out = models.EndpointOut{}
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
