package svix_test

import (
	"encoding/json"
	"testing"
	svix "github.com/svix/svix-webhooks/go"
)

func TestMessageInRawSerialization(t *testing.T) {
	msgIn := svix.NewMessageInRaw("bar.foo", "{}", nil)
	msgInJsonBytes, _ := json.Marshal(msgIn)
	msgInJson := string(msgInJsonBytes)

	expectedJson := `{"eventType":"bar.foo","payload":{},"transformationsParams":{"rawPayload":"{}"}}`

	if msgInJson != expectedJson {
		t.Errorf("Wrong serialization: %s", msgInJson)
	}
}
