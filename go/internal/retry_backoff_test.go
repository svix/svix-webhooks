package internal

import (
	"context"
	"errors"
	"io"
	"net/http"
	"strings"
	"sync/atomic"
	"testing"
	"time"
)

type backoffTransport func(*http.Request) (*http.Response, error)

func (transport backoffTransport) RoundTrip(request *http.Request) (*http.Response, error) {
	return transport(request)
}

type backoffBody struct {
	io.Reader
	closed func()
}

func (body *backoffBody) Close() error {
	body.closed()
	return nil
}

func TestRetryBackoffCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	var calls atomic.Int32
	var closes atomic.Int32
	discarded := make(chan struct{})
	client := &SvixHttpClient{
		RetrySchedule: []time.Duration{time.Hour},
		HTTPClient: &http.Client{Transport: backoffTransport(func(request *http.Request) (*http.Response, error) {
			calls.Add(1)
			return &http.Response{
				StatusCode: http.StatusServiceUnavailable,
				Body: &backoffBody{Reader: strings.NewReader("retry later"), closed: func() {
					if closes.Add(1) == 1 {
						close(discarded)
					}
				}},
				Header:  make(http.Header),
				Request: request,
			}, nil
		})},
	}
	request, err := http.NewRequestWithContext(ctx, http.MethodGet, "https://example.test/", nil)
	if err != nil {
		t.Fatal(err)
	}
	done := make(chan error, 1)
	go func() {
		_, err := executeRequestWithRetries(client, request)
		done <- err
	}()
	select {
	case <-discarded:
	case <-time.After(2 * time.Second):
		t.Fatal("first response was not discarded")
	}
	cancel()
	select {
	case err := <-done:
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("expected cancellation, got %v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("cancellation did not interrupt retry backoff")
	}
	if calls.Load() != 1 {
		t.Fatalf("request retried after cancellation: %d calls", calls.Load())
	}
	if closes.Load() != 1 {
		t.Fatalf("discarded body closed %d times", closes.Load())
	}
}

func TestRetryBackoffDeadline(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()
	var calls atomic.Int32
	client := &SvixHttpClient{
		RetrySchedule: []time.Duration{time.Hour},
		HTTPClient: &http.Client{Transport: backoffTransport(func(*http.Request) (*http.Response, error) {
			calls.Add(1)
			return nil, io.ErrUnexpectedEOF
		})},
	}
	request, err := http.NewRequestWithContext(ctx, http.MethodGet, "https://example.test/", nil)
	if err != nil {
		t.Fatal(err)
	}
	done := make(chan error, 1)
	go func() {
		_, err := executeRequestWithRetries(client, request)
		done <- err
	}()
	select {
	case err := <-done:
		if !errors.Is(err, context.DeadlineExceeded) {
			t.Fatalf("expected deadline error, got %v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("deadline did not interrupt retry backoff")
	}
	if calls.Load() != 1 {
		t.Fatalf("request retried after deadline: %d calls", calls.Load())
	}
}

func TestRetryBackoffDoesNotDispatchACanceledRequest(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	var calls int
	client := &SvixHttpClient{
		RetrySchedule: []time.Duration{0},
		HTTPClient: &http.Client{Transport: backoffTransport(func(request *http.Request) (*http.Response, error) {
			calls++
			status := http.StatusOK
			body := io.ReadCloser(io.NopCloser(strings.NewReader("success")))
			if calls == 1 {
				status = http.StatusServiceUnavailable
				body = &backoffBody{Reader: strings.NewReader("retry"), closed: cancel}
			}
			return &http.Response{StatusCode: status, Body: body, Header: make(http.Header), Request: request}, nil
		})},
	}
	request, err := http.NewRequestWithContext(ctx, http.MethodGet, "https://example.test/", nil)
	if err != nil {
		t.Fatal(err)
	}
	response, err := executeRequestWithRetries(client, request)
	if response != nil {
		_ = response.Body.Close()
	}
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("expected cancellation, got response %v, error %v", response, err)
	}
	if calls != 1 {
		t.Fatalf("dispatched %d calls; wanted only the first", calls)
	}
}

func TestRetryBackoffWithoutCancellationPreservesRequestAndFinalBody(t *testing.T) {
	for _, delay := range []time.Duration{0, -time.Millisecond, time.Millisecond} {
		t.Run(delay.String(), func(t *testing.T) {
			var bodies []string
			var closes int
			client := &SvixHttpClient{
				RetrySchedule: []time.Duration{delay},
				HTTPClient: &http.Client{Transport: backoffTransport(func(request *http.Request) (*http.Response, error) {
					body, err := io.ReadAll(request.Body)
					if err != nil {
						return nil, err
					}
					_ = request.Body.Close()
					bodies = append(bodies, string(body))
					if request.Header.Get("idempotency-key") != "stable-key" {
						t.Error("idempotency key changed")
					}
					status := http.StatusServiceUnavailable
					if len(bodies) == 2 {
						status = http.StatusBadRequest
						if request.Header.Get("svix-retry-count") != "1" {
							t.Error("retry count not preserved")
						}
					}
					return &http.Response{
						StatusCode: status,
						Body:       &backoffBody{Reader: strings.NewReader("response body"), closed: func() { closes++ }},
						Header:     make(http.Header), Request: request,
					}, nil
				})},
			}
			request, err := http.NewRequestWithContext(context.Background(), http.MethodPost, "https://example.test/", strings.NewReader("payload"))
			if err != nil {
				t.Fatal(err)
			}
			request.Header.Set("idempotency-key", "stable-key")
			response, err := executeRequestWithRetries(client, request)
			if err != nil {
				t.Fatal(err)
			}
			defer response.Body.Close()
			if response.StatusCode != http.StatusBadRequest {
				t.Fatalf("final status %d", response.StatusCode)
			}
			if len(bodies) != 2 || bodies[0] != "payload" || bodies[1] != "payload" {
				t.Fatalf("bodies not replayed: %q", bodies)
			}
			if closes != 1 {
				t.Fatalf("expected only discarded response closed, got %d", closes)
			}
			body, err := io.ReadAll(response.Body)
			if err != nil || string(body) != "response body" {
				t.Fatalf("final body lost: %q, %v", body, err)
			}
		})
	}
}
