package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"

	svix "github.com/svix/svix-webhooks/go"
	"github.com/svix/svix-webhooks/go/internalapi"
	"github.com/svix/svix-webhooks/go/models"
)

type langFixtures struct {
	AppID            string `json:"appId"`
	V1HTTP           string `json:"v1Http"`
	V1Poller         string `json:"v1Poller"`
	V2HTTP           string `json:"v2Http"`
	V2Poller         string `json:"v2Poller"`
	V2PollerExisting string `json:"v2PollerExisting"`
}

type fixturesFile struct {
	ServerURL  string `json:"serverUrl"`
	OrgToken   string `json:"orgToken"`
	EventType  string `json:"eventType"`
	HTTPURL    string `json:"httpUrl"`
	ConsumerID string `json:"consumerId"`
	Languages  struct {
		Go langFixtures `json:"go"`
	} `json:"languages"`
}

func loadFixtures(path string) (fixturesFile, error) {
	var fx fixturesFile
	b, err := os.ReadFile(path)
	if err != nil {
		return fx, err
	}
	if err := json.Unmarshal(b, &fx); err != nil {
		return fx, err
	}
	return fx, nil
}

func sendMessage(serverURL, orgToken, appID, eventType, src string) error {
	body, err := json.Marshal(map[string]any{
		"eventType": eventType,
		"payload":   map[string]any{"ok": true, "src": src},
	})
	if err != nil {
		return err
	}
	url := strings.TrimRight(serverURL, "/") + "/api/v1/app/" + appID + "/msg/"
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return err
	}
	req.Header.Set("Authorization", "Bearer "+orgToken)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	respBody, _ := io.ReadAll(resp.Body)
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("POST msg status %d: %s", resp.StatusCode, strings.TrimSpace(string(respBody)))
	}
	return nil
}

func pollOpts() *internalapi.MessagePollerv2ConsumerPollOptions {
	start := models.STARTINGPOSITION_EARLIEST
	lease := uint64(2000)
	return &internalapi.MessagePollerv2ConsumerPollOptions{
		StartingPosition: &start,
		LeaseDurationMs:  &lease,
	}
}

func receiveSrc(ctx context.Context, consumer *svix.AutoConfigConsumer, consumerID, src string) (*models.PollerV2MessageOut, error) {
	var lastErr error
	for i := 0; i < 10; i++ {
		out, err := consumer.Receive(ctx, consumerID, pollOpts())
		if err != nil {
			lastErr = err
			time.Sleep(2 * time.Second)
			continue
		}
		for i := range out.Data {
			msg := &out.Data[i]
			if got, _ := msg.Payload["src"].(string); got == src {
				return msg, nil
			}
		}
		lastErr = fmt.Errorf("no message with src=%s (got %d)", src, len(out.Data))
		time.Sleep(2 * time.Second)
	}
	return nil, lastErr
}

func runHTTP(ctx context.Context, token, httpURL, eventType string) (string, error) {
	ac, err := svix.NewAutoConfig(token, models.EndpointIn{
		Url:        httpURL,
		EventTypes: []string{eventType},
	})
	if err != nil {
		return "", fmt.Errorf("NewAutoConfig: %w", err)
	}
	ep, err := ac.Subscribe(ctx)
	if err != nil {
		return "", fmt.Errorf("Subscribe: %w", err)
	}
	if ep.Url != httpURL {
		return "", fmt.Errorf("url mismatch got %q", ep.Url)
	}
	return ep.Id, nil
}

func runPoller(ctx context.Context, token, consumerID, serverURL, orgToken, appID, eventType, src string) (string, error) {
	consumer, err := svix.NewAutoConfigConsumer(token, models.SinkInCommon{
		EventTypes: []string{eventType},
	})
	if err != nil {
		return "", fmt.Errorf("NewAutoConfigConsumer: %w", err)
	}
	dest, err := consumer.Subscribe(ctx)
	if err != nil {
		return destID(dest), fmt.Errorf("Subscribe: %w", err)
	}
	if err := sendMessage(serverURL, orgToken, appID, eventType, src); err != nil {
		return dest.Id, fmt.Errorf("send: %w", err)
	}
	msg, err := receiveSrc(ctx, consumer, consumerID, src)
	if err != nil {
		return dest.Id, fmt.Errorf("Receive: %w", err)
	}
	if err := consumer.Commit(ctx, consumerID, msg.Offset, nil); err != nil {
		return dest.Id, fmt.Errorf("Commit offset %d: %w", msg.Offset, err)
	}
	return dest.Id, nil
}

func runPollerExisting(ctx context.Context, token, consumerID, serverURL, orgToken, appID, eventType, src string) (string, error) {
	binder, err := svix.NewAutoConfigConsumer(token, models.SinkInCommon{
		EventTypes: []string{eventType},
	})
	if err != nil {
		return "", fmt.Errorf("NewAutoConfigConsumer binder: %w", err)
	}
	dest, err := binder.Subscribe(ctx)
	if err != nil {
		return destID(dest), fmt.Errorf("Subscribe: %w", err)
	}
	consumer, err := svix.NewAutoConfigConsumer(token, models.SinkInCommon{
		EventTypes: []string{eventType},
	})
	if err != nil {
		return dest.Id, fmt.Errorf("NewAutoConfigConsumer receiver: %w", err)
	}
	if err := sendMessage(serverURL, orgToken, appID, eventType, src); err != nil {
		return dest.Id, fmt.Errorf("send: %w", err)
	}
	msg, err := receiveSrc(ctx, consumer, consumerID, src)
	if err != nil {
		return dest.Id, fmt.Errorf("Receive: %w", err)
	}
	if err := consumer.Commit(ctx, consumerID, msg.Offset, nil); err != nil {
		return dest.Id, fmt.Errorf("Commit offset %d: %w", msg.Offset, err)
	}
	return dest.Id, nil
}

func destID(dest *models.DestinationOut) string {
	if dest == nil {
		return ""
	}
	return dest.Id
}

func errDetail(err error) string {
	var se *svix.Error
	if errors.As(err, &se) {
		body := strings.TrimSpace(string(se.Body()))
		if body != "" {
			return se.Error() + " " + body
		}
	}
	return err.Error()
}

func line(ok bool, detail string) string {
	if ok {
		return "PASS — " + detail
	}
	return "FAIL — " + detail
}

func main() {
	fx, err := loadFixtures(os.Getenv("AUTOCONFIG_FIXTURES"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "load fixtures: %v\n", err)
		os.Exit(1)
	}
	goFx := fx.Languages.Go
	ctx := context.Background()

	aID, aErr := runHTTP(ctx, goFx.V1HTTP, fx.HTTPURL, fx.EventType)
	bID, bErr := runHTTP(ctx, goFx.V2HTTP, fx.HTTPURL, fx.EventType)
	cID, cErr := runPoller(ctx, goFx.V1Poller, fx.ConsumerID, fx.ServerURL, fx.OrgToken, goFx.AppID, fx.EventType, "go-v1")
	dID, dErr := runPoller(ctx, goFx.V2Poller, fx.ConsumerID, fx.ServerURL, fx.OrgToken, goFx.AppID, fx.EventType, "go-v2")
	eID, eErr := runPollerExisting(ctx, goFx.V2PollerExisting, fx.ConsumerID, fx.ServerURL, fx.OrgToken, goFx.AppID, fx.EventType, "go-e")

	fmt.Println("LANG: go")
	fmt.Println("A: " + line(aErr == nil, httpDetail("v1 HTTP", aID, aErr)))
	fmt.Println("B: " + line(bErr == nil, httpDetail("v2 HTTP", bID, bErr)))
	fmt.Println("C: " + line(cErr == nil, pollerDetail("v1 poller", cID, cErr)))
	fmt.Println("D: " + line(dErr == nil, pollerDetail("v2 poller", dID, dErr)))
	fmt.Println("E: " + line(eErr == nil, pollerDetail("v2 receive without subscribe on receiver", eID, eErr)))
	fmt.Println("Notes: replace github.com/svix/svix-webhooks => repo root")
}

func httpDetail(label, id string, err error) string {
	if err == nil {
		return label + " " + id
	}
	return label + ": " + errDetail(err)
}

func pollerDetail(label, id string, err error) string {
	detail := label
	if id != "" {
		detail += " " + id
	}
	if err == nil {
		return detail + " received+committed"
	}
	return detail + ": " + errDetail(err)
}
