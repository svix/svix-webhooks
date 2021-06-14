package svix

import (
	"encoding/base64"
	"fmt"
	"net/http"
	"testing"
	"time"
)

var defaultMsgID = "msg_p5jXN8AQM9LWM0D4loKWxJek"
var defaultPayload = []byte(`{"test": 2432232314}`)
var defaultSecret = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"

type testPayload struct {
	id        string
	timestamp time.Time
	header    http.Header
	secret    string
	payload   []byte
	signature []byte
}

func newTestPayload(timestamp time.Time) *testPayload {
	tp := &testPayload{}
	tp.id = defaultMsgID
	tp.timestamp = timestamp

	tp.payload = defaultPayload
	tp.secret = defaultSecret
	toSign := fmt.Sprintf("%s.%d.%s", tp.id, tp.timestamp.Unix(), tp.payload)

	secret, _ := base64.StdEncoding.DecodeString(tp.secret)
	tp.signature = sign(secret, toSign)

	tp.header = http.Header{}
	tp.header.Set("svix-id", tp.id)
	tp.header.Set("svix-signature", fmt.Sprintf("v1,%s", tp.signature))
	tp.header.Set("svix-timestamp", fmt.Sprint(tp.timestamp.Unix()))

	return tp
}

func TestWebhook(t *testing.T) {

	testCases := []struct {
		name          string
		testPayload   *testPayload
		modifyPayload func(*testPayload)
		expectedErr   bool
	}{
		{
			name:        "valid signature is valid",
			testPayload: newTestPayload(time.Now()),
			expectedErr: false,
		},
		{
			name:        "missing id returns error",
			testPayload: newTestPayload(time.Now()),
			modifyPayload: func(tp *testPayload) {
				tp.header.Del("svix-id")
			},
			expectedErr: true,
		},
		{
			name:        "missing timestamp returns error",
			testPayload: newTestPayload(time.Now()),
			modifyPayload: func(tp *testPayload) {
				tp.header.Del("svix-timestamp")
			},
			expectedErr: true,
		},
		{
			name:        "missing signature returns error",
			testPayload: newTestPayload(time.Now()),
			modifyPayload: func(tp *testPayload) {
				tp.header.Del("svix-signature")
			},
			expectedErr: true,
		},
		{
			name:        "invalid signature is invalid",
			testPayload: newTestPayload(time.Now()),
			modifyPayload: func(tp *testPayload) {
				tp.header.Set("svix-signature", "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=")
			},
			expectedErr: true,
		},
		{
			name:        "old timestamp fails",
			testPayload: newTestPayload(time.Now().Add(tolerance * -1)),
			expectedErr: true,
		},
		{
			name:        "new timestamp fails",
			testPayload: newTestPayload(time.Now().Add(tolerance + time.Second)),
			expectedErr: true,
		},
	}

	for _, tc := range testCases {
		if tc.modifyPayload != nil {
			tc.modifyPayload(tc.testPayload)
		}

		wh, err := NewWebhook(tc.testPayload.secret)
		if err != nil {
			t.Error(err)
			continue
		}
		err = wh.Verify(tc.testPayload.payload, tc.testPayload.header)
		if err != nil && !tc.expectedErr {
			t.Errorf("%s: failed with err %s but shouldn't have", tc.name, err.Error())
		} else if err == nil && tc.expectedErr {
			t.Errorf("%s: didn't error but should have", tc.name)
		}
	}
}
