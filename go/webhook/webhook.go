package webhook

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"net/http"
	"strings"
)

var base64enc = base64.StdEncoding

type Webhook struct {
	key []byte
}

func New(secret string) (*Webhook, error) {
	key, err := base64enc.DecodeString(secret)
	if err != nil {
		return nil, err
	}
	return &Webhook{
		key: key,
	}, nil
}

func (wh *Webhook) Verify(payload string, headers http.Header) error {
	msgID := headers.Get("svix-id")
	msgSignature := headers.Get("svix-signature")
	msgTimestamp := headers.Get("svix-timestamp")

	if msgID == "" || msgSignature == "" || msgTimestamp == "" {
		return fmt.Errorf("Missing Required Headers")
	}

	toSign := fmt.Sprintf("%s.%s.%s", msgID, msgTimestamp, payload)
	expectedSignature := sign(wh.key, toSign)
	passedSignatures := strings.Split(msgSignature, " ")

	for _, versionedSignature := range passedSignatures {
		sigParts := strings.Split(versionedSignature, ",")
		if len(sigParts) < 2 {
			continue
		}
		version := sigParts[0]
		signature := sigParts[1]

		if version != "v1" {
			continue
		}

		if signature == expectedSignature {
			return nil
		}
	}
	return fmt.Errorf("No matching signature found")
}

func sign(key []byte, toSign string) string {
	h := hmac.New(sha256.New, key)
	h.Write([]byte(toSign))
	return base64enc.EncodeToString((h.Sum(nil)))
}
