package svix_test

import (
	"context"
	"net/http"
	"net/url"
	"reflect"
	"sort"
	"strings"
	"testing"

	"github.com/jarcoal/httpmock"
	svix "github.com/svix/svix-webhooks/go"
	"github.com/svix/svix-webhooks/go/models"
)

func newMockClient() svix.Svix {
	url, err := url.Parse("http://testapi.test")
	if err != nil {
		panic(err)
	}
	svx, err := svix.New("randomToken", &svix.SvixOptions{ServerUrl: url})
	if err != nil {
		panic(err)
	}
	return *svx

}
func assertExpectedError(t *testing.T, err error, expected string) {
	if err == nil {
		t.Error("Expected to get error")
	}
	if err.Error() != expected {
		t.Errorf("Unexpected error, expected: `%s` got `%s`", expected, err.Error())
	}
}

func TestReqIdHeaderIsSetCorrectly(t *testing.T) {
	svx := newMockClient()
	httpmock.Activate()
	defer httpmock.DeactivateAndReset()
	var req_id string

	httpmock.RegisterResponder("GET", "http://testapi.test/api/v1/app",
		func(r *http.Request) (*http.Response, error) {
			if r.Header.Get("svix-req-id") == "" {
				t.Errorf("Requests must send a `svix-req-id` header")
			}
			if req_id == "" {
				req_id = r.Header.Get("svix-req-id")
			} else {
				if req_id != r.Header.Get("svix-req-id") {
					t.Errorf("Requests in the retry loop must have the same svix-req-id, expected: `%s` got: `%s`", req_id, r.Header.Get("svix-req-id"))
				}
			}
			return httpmock.NewStringResponse(500, ""), nil
		},
	)

	_, err := svx.Application.List(context.TODO(), nil)
	assertExpectedError(t, err, "status code 500")

	if httpmock.GetTotalCallCount() != 4 {
		t.Errorf("Expected client to send 4 requests (1 original and 3 retries) got %v", httpmock.GetTotalCallCount())
	}

}

func TestRetryCountHeadersIsSetCorrectly(t *testing.T) {
	svx := newMockClient()
	httpmock.Activate()
	defer httpmock.DeactivateAndReset()

	retryCounts := []string{}

	httpmock.RegisterResponder("GET", "http://testapi.test/api/v1/app",
		func(r *http.Request) (*http.Response, error) {
			if r.Header.Get("svix-retry-count") != "" {
				retryCounts = append(retryCounts, r.Header.Get("svix-retry-count"))
			}
			return httpmock.NewStringResponse(500, ""), nil
		},
	)

	_, err := svx.Application.List(context.TODO(), nil)
	assertExpectedError(t, err, "status code 500")

	if httpmock.GetTotalCallCount() != 4 {
		t.Errorf("Expected client to send 4 requests (1 original and 3 retries) got %v", httpmock.GetTotalCallCount())
	}
	if len(retryCounts) != 3 {
		t.Errorf("Expected client to send 3 requests with svix-retry-count header, got: %v", len(retryCounts))
	}

	sort.Slice(retryCounts, func(i, j int) bool { return strings.ToLower(retryCounts[i]) < strings.ToLower(retryCounts[j]) })
	if !reflect.DeepEqual(retryCounts, []string{"1", "2", "3"}) {
		t.Errorf("Expected client to incorrect the svix-retry-count header by one each request, got: %v", retryCounts)

	}
}

func TestOptionsSerialization(t *testing.T) {
	svx := newMockClient()
	httpmock.Activate()
	defer httpmock.DeactivateAndReset()

	httpmock.RegisterResponder("GET", "http://testapi.test/api/v1/app",
		func(r *http.Request) (*http.Response, error) {
			if !reflect.DeepEqual(r.URL.RawQuery, "iterator=asd%5E%26%2A1223&limit=12&order=ascending") {
				t.Errorf("Unexpected ApplicationListOptions serialization, got: %v", r.URL.RawQuery)
			}

			return httpmock.NewStringResponse(200, ""), nil
		},
	)
	limit := uint64(12)
	order := models.ORDERING_ASCENDING
	iter := "asd^&*1223"
	listOpts := svix.ApplicationListOptions{
		Limit:    &limit,
		Order:    &order,
		Iterator: &iter,
	}
	_, err := svx.Application.List(context.TODO(), &listOpts)
	assertExpectedError(t, err, "unexpected end of JSON input")

}

func TestQueryParamListSerialization(t *testing.T) {
	svx := newMockClient()
	httpmock.Activate()
	defer httpmock.DeactivateAndReset()

	httpmock.RegisterResponder("GET", "http://testapi.test/api/v1/app/random_app_id/msg",
		func(r *http.Request) (*http.Response, error) {
			if !reflect.DeepEqual(r.URL.RawQuery, "event_types=asd13%2C123asd") {
				t.Errorf("Unexpected MessageListOptions serialization, got: %v", r.URL.RawQuery)
			}

			return httpmock.NewStringResponse(200, ""), nil
		},
	)
	listOpts := svix.MessageListOptions{
		EventTypes: &[]string{"asd13", "123asd"},
	}
	_, err := svx.Message.List(context.TODO(), "random_app_id", &listOpts)
	assertExpectedError(t, err, "unexpected end of JSON input")
}
