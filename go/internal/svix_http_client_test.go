package internal

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"log"
	"net/http"
	"net/http/httptest"
	"sync/atomic"
	"testing"
	"time"

	"github.com/svix/svix-webhooks/go/models"
)

func Test_executeRequestWithRetries(t *testing.T) {
	tests := []struct {
		name           string
		failFirst      bool
		expectedCalls  int32
		expectedStatus int
	}{
		{
			name:           "request succeeds with retry after initial failure",
			failFirst:      true,
			expectedCalls:  2,
			expectedStatus: http.StatusOK,
		},
		{
			name:           "request succeeds immediately without retry",
			failFirst:      false,
			expectedCalls:  1,
			expectedStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Given ... we have a test server
			var requestCount int32
			var receivedBodies []string

			// And ... we have a test payload
			payload := map[string]any{
				"test": "test",
				"blah": "blah",
			}

			// And ... we have an event type and message id
			eventType := "someEvent"
			msgId := "msg_123"

			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				count := atomic.AddInt32(&requestCount, 1)

				body, err := io.ReadAll(r.Body)
				if err != nil {
					t.Errorf("failed to read request body: %v", err)
					return
				}
				receivedBodies = append(receivedBodies, string(body))

				t.Logf("Request %d: Content-Length=%s, Body-Length=%d",
					count, r.Header.Get("Content-Length"), len(body))
				t.Logf("request %d body: %s", count, string(body))

				if tt.failFirst && count == 1 {
					w.WriteHeader(http.StatusInternalServerError)
					_, err := w.Write([]byte(`{"error": "server error"}`))
					if err != nil {
						t.Errorf("failed to write status code - error: %v", err)
						return
					}
					return
				}

				w.Header().Set("Content-Type", "application/json")
				w.WriteHeader(tt.expectedStatus)
				response := models.MessageOut{
					Id:        msgId,
					EventType: eventType,
					Payload:   payload,
				}
				err = json.NewEncoder(w).Encode(response)
				if err != nil {
					t.Errorf("failed to encode json - error: %v", err)
				}
			}))
			defer server.Close()

			// And ... we have a svix http client
			client := &SvixHttpClient{
				HTTPClient:     &http.Client{},
				BaseURL:        server.URL,
				RetrySchedule:  []time.Duration{10 * time.Millisecond}, // Quick retry for test
				DefaultHeaders: map[string]string{},
				Debug:          true,
			}

			// And ... a test message in
			messageIn := models.MessageIn{
				EventType: eventType,
				Payload:   payload,
			}

			// When ... we execute the request
			jsonBody, err := json.Marshal(messageIn)
			if err != nil {
				t.Fatalf("Failed to marshal request body: %v", err)
			}

			req, err := http.NewRequest("POST", server.URL+"/api/v1/app/test_app/msg", bytes.NewBuffer(jsonBody))
			if err != nil {
				t.Fatalf("Failed to create request: %v", err)
			}
			req.Header.Set("Content-Type", "application/json")

			resp, err := executeRequestWithRetries(client, req)
			if err != nil {
				t.Fatalf("request failed: %v", err)
			}

			defer func() {
				err := resp.Body.Close()
				if err != nil {
					log.Fatalf("error closing response body: %v", err)
				}
			}()

			// Then ... the response should be decoded without error
			var result models.MessageOut
			if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
				t.Fatalf("Failed to decode response: %v", err)
			}

			// And ... our request count should match the expected number of calls
			if requestCount != tt.expectedCalls {
				t.Errorf("Expected %d requests, got %d", tt.expectedCalls, requestCount)
			}

			// And ... we should have the same number of request bodies
			if len(receivedBodies) != int(tt.expectedCalls) {
				t.Fatalf("Expected %d request bodies, got %d", tt.expectedCalls, len(receivedBodies))
			}

			// And ... the bodies should be set as expected
			for i, body := range receivedBodies {
				if len(body) == 0 {
					t.Errorf("Request body %d is empty", i+1)
				}

				var receivedMsg models.MessageIn
				if err := json.Unmarshal([]byte(body), &receivedMsg); err != nil {
					t.Errorf("Failed to unmarshal request body %d: %v", i+1, err)
				}
				if receivedMsg.EventType != messageIn.EventType {
					t.Errorf("Request body %d mismatch: expected %s, got %s",
						i+1, messageIn.EventType, receivedMsg.EventType)
				}
			}

			// And ... if there are multiple calls then the bodies should match
			if tt.expectedCalls > 1 {
				if receivedBodies[0] != receivedBodies[1] {
					t.Error("Request bodies don't match between retries")
					t.Errorf("First body length: %d, Second body length: %d",
						len(receivedBodies[0]), len(receivedBodies[1]))
					t.Errorf("First body: %s", receivedBodies[0])
					t.Errorf("Second body: %s", receivedBodies[1])
				}
			}
		})
	}
}

func Test_requestURL(t *testing.T) {
	tests := []struct {
		name       string
		baseURL    string
		path       string
		pathParams map[string]string
		expected   string
	}{
		{
			name:     "trailing slash is stripped",
			baseURL:  "https://api.example.com/",
			path:     "/api/v1/app",
			expected: "https://api.example.com/api/v1/app",
		},
		{
			name:     "multiple trailing slashes are stripped",
			baseURL:  "https://api.example.com///",
			path:     "/api/v1/app",
			expected: "https://api.example.com/api/v1/app",
		},
		{
			name:     "a path prefix is kept",
			baseURL:  "https://api.example.com/prefix/",
			path:     "/api/v1/app",
			expected: "https://api.example.com/prefix/api/v1/app",
		},
		{
			name:     "a base URL without a trailing slash is unchanged",
			baseURL:  "https://api.example.com",
			path:     "/api/v1/app",
			expected: "https://api.example.com/api/v1/app",
		},
		{
			name:       "path params are still substituted",
			baseURL:    "https://api.example.com/",
			path:       "/api/v1/app/{app_id}/msg",
			pathParams: map[string]string{"app_id": "app_123"},
			expected:   "https://api.example.com/api/v1/app/app_123/msg",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			client := &SvixHttpClient{BaseURL: tt.baseURL}
			got := requestURL(client, tt.path, tt.pathParams)
			if got != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, got)
			}
		})
	}
}

func Test_executeRequestStripsTrailingSlashFromBaseURL(t *testing.T) {
	// Given ... a test server that records the path it was asked for
	var requestedPath string
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		requestedPath = r.URL.Path
		w.Header().Set("Content-Type", "application/json")
		err := json.NewEncoder(w).Encode(models.MessageOut{Id: "msg_123"})
		if err != nil {
			t.Errorf("failed to encode json - error: %v", err)
		}
	}))
	defer server.Close()

	// And ... a client whose base URL ends in a slash
	client := &SvixHttpClient{
		HTTPClient:     &http.Client{},
		BaseURL:        server.URL + "/",
		DefaultHeaders: map[string]string{},
	}

	// When ... we execute a request
	_, err := ExecuteRequest[any, models.MessageOut](
		context.Background(), client, "GET", "/api/v1/app/{app_id}/msg",
		map[string]string{"app_id": "app_123"}, nil, nil, nil,
	)
	if err != nil {
		t.Fatalf("request failed: %v", err)
	}

	// Then ... the server sees a single slash between the base URL and the path
	expected := "/api/v1/app/app_123/msg"
	if requestedPath != expected {
		t.Errorf("expected path %q, got %q", expected, requestedPath)
	}
}
