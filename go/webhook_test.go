package svix_test

import (
	"net/http"
	"testing"

	svix "github.com/svixhq/svix-libs/go"
)

func TestWebhook(t *testing.T) {
	testCases := []struct {
		name        string
		secret      string
		payload     []byte
		headers     http.Header
		expectedErr bool
	}{
		{
			name:    "valid signature is valid",
			secret:  "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw",
			payload: []byte(`{"test": 2432232314}`),
			headers: http.Header{
				"Svix-Id":        []string{"msg_p5jXN8AQM9LWM0D4loKWxJek"},
				"Svix-Timestamp": []string{"1614265330"},
				"Svix-Signature": []string{"v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="},
			},
			expectedErr: false,
		},
		{
			name:    "missing id returns error",
			secret:  "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw",
			payload: []byte(`{"test": 2432232314}`),
			headers: http.Header{
				"Svix-Timestamp": []string{"1614265330"},
				"Svix-Signature": []string{"v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="},
			},
			expectedErr: true,
		},
		{
			name:    "missing timestamp returns error",
			secret:  "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw",
			payload: []byte(`{"test": 2432232314}`),
			headers: http.Header{
				"Svix-Id":        []string{"msg_p5jXN8AQM9LWM0D4loKWxJek"},
				"Svix-Signature": []string{"v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="},
			},
			expectedErr: true,
		},
		{
			name:    "missing signature returns error",
			secret:  "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw",
			payload: []byte(`{"test": 2432232314}`),
			headers: http.Header{
				"Svix-Id":        []string{"msg_p5jXN8AQM9LWM0D4loKWxJek"},
				"Svix-Timestamp": []string{"1614265330"},
			},
			expectedErr: true,
		},
		{
			name:    "invalid signature is invalid",
			secret:  "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw",
			payload: []byte(`{"test": 2432232315}`),
			headers: http.Header{
				"Svix-Id":        []string{"msg_p5jXN8AQM9LWM0D4loKWxJek"},
				"Svix-Timestamp": []string{"1614265330"},
				"Svix-Signature": []string{"v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="},
			},
			expectedErr: true,
		},
	}

	for _, tc := range testCases {
		wh, err := svix.NewWebhook(tc.secret)
		if err != nil {
			t.Error(err)
			continue
		}
		err = wh.Verify(tc.payload, tc.headers)
		if err != nil && !tc.expectedErr {
			t.Errorf("%s: failed with err %s but shouldn't have", tc.name, err.Error())
		} else if err == nil && tc.expectedErr {
			t.Errorf("%s: didn't error but should have", tc.name)
		}
	}
}
