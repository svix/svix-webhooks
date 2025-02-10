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
	if err == nil {
		t.Errorf("Client did not return an error for status code 500")
	}

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
	if err == nil {
		t.Errorf("Client did not return an error for status code 500")
	}

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
