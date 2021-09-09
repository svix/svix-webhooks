package svix

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"net/http"
	"strconv"
	"strings"
	"time"
)

var base64enc = base64.StdEncoding

type Webhook struct {
	key []byte
}

const webhookSecretPrefix = "whsec_"

var tolerance time.Duration = 5 * time.Minute

var (
	errRequiredHeaders     = fmt.Errorf("Missing Required Headers")
	errInvalidHeaders      = fmt.Errorf("Invalid Signature Headers")
	errNoMatchingSignature = fmt.Errorf("No matching signature found")
	errMessageTooOld       = fmt.Errorf("Message timestamp too old")
	errMessageTooNew       = fmt.Errorf("Message timestamp too new")
)

func NewWebhook(secret string) (*Webhook, error) {
	key, err := base64enc.DecodeString(strings.TrimPrefix(secret, webhookSecretPrefix))
	if err != nil {
		return nil, err
	}
	return &Webhook{
		key: key,
	}, nil
}

func (wh *Webhook) Verify(payload []byte, headers http.Header) error {
	msgId := headers.Get("svix-id")
	msgSignature := headers.Get("svix-signature")
	msgTimestamp := headers.Get("svix-timestamp")

	if msgId == "" || msgSignature == "" || msgTimestamp == "" {
		msgId = headers.Get("webhook-id")
		msgSignature = headers.Get("webhook-signature")
		msgTimestamp = headers.Get("webhook-timestamp")
		if msgId == "" || msgSignature == "" || msgTimestamp == "" {
			return errRequiredHeaders
		}
	}

	// enforce timestamp tolerance
	timestamp, err := verifyTimestamp(msgTimestamp)
	if err != nil {
		return err
	}

	computedSignature, err := wh.Sign(msgId, timestamp, payload)
	if err != nil {
		return err
	}
	expectedSignature := []byte(strings.Split(computedSignature, ",")[1])

	passedSignatures := strings.Split(msgSignature, " ")
	for _, versionedSignature := range passedSignatures {
		sigParts := strings.Split(versionedSignature, ",")
		if len(sigParts) < 2 {
			continue
		}
		version := sigParts[0]
		signature := []byte(sigParts[1])

		if version != "v1" {
			continue
		}

		if hmac.Equal(signature, expectedSignature) {
			return nil
		}
	}
	return errNoMatchingSignature
}

func (wh *Webhook) Sign(msgId string, timestamp time.Time, payload []byte) (string, error) {
	toSign := fmt.Sprintf("%s.%d.%s", msgId, timestamp.Unix(), payload)

	h := hmac.New(sha256.New, wh.key)
	h.Write([]byte(toSign))
	sig := make([]byte, base64enc.EncodedLen(h.Size()))
	base64enc.Encode(sig, h.Sum(nil))
	return fmt.Sprintf("v1,%s", sig), nil

}

func verifyTimestamp(timestampHeader string) (time.Time, error) {
	now := time.Now()
	timeInt, err := strconv.ParseInt(timestampHeader, 10, 64)
	if err != nil {
		return time.Time{}, errInvalidHeaders
	}
	timestamp := time.Unix(timeInt, 0)

	if now.Sub(timestamp) > tolerance {
		return time.Time{}, errMessageTooOld
	}
	if timestamp.Unix() > now.Add(tolerance).Unix() {
		return time.Time{}, errMessageTooNew
	}

	return timestamp, nil
}
