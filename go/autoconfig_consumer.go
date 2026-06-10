package svix

import (
	"context"
	"net/url"

	"github.com/svix/svix-webhooks/go/internalapi"
	"github.com/svix/svix-webhooks/go/models"
)

// AutoConfigConsumer decodes an auto-configuration token for polling sink consumers.
type AutoConfigConsumer struct {
	appID      string
	sinkID     string
	sinkIn     models.SinkInCommon
	svix       *internalapi.InternalSvix
}

// NewAutoConfigConsumer parses a v1 auto-config token and prepares consumer helpers.
func NewAutoConfigConsumer(token string, sinkIn models.SinkInCommon) (*AutoConfigConsumer, error) {
	content, err := DecodeAutoConfigTokenV1(token)
	if err != nil {
		return nil, err
	}

	serverURL, err := url.Parse(content.ServerURL)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	svx, err := internalapi.New(content.TokenPlaintext, serverURL, false, nil)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	return &AutoConfigConsumer{
		appID:  content.AppID,
		sinkID: content.EndpointID,
		sinkIn: sinkIn,
		svix:   svx,
	}, nil
}

// Subscribe registers or updates the polling sink via the auto-config API.
func (a *AutoConfigConsumer) Subscribe(ctx context.Context) (*models.EndpointOut, error) {
	return a.svix.Endpoint.AutoConfig.Update(
		ctx,
		a.appID,
		a.sinkID,
		models.SubscribeIn{
			Sink: &models.AutoConfigSinkType{
				Type:   models.AutoConfigSinkTypeTypePoller,
				Config: a.sinkIn,
			},
		},
	)
}

// Receive polls messages from the sink for the given consumer.
func (a *AutoConfigConsumer) Receive(
	ctx context.Context,
	consumerID string,
	options *internalapi.MessagePollerv2ConsumerPollOptions,
) (*models.PollerV2PollOut, error) {
	return a.svix.Message.Pollerv2.ConsumerPoll(
		ctx,
		a.appID,
		a.sinkID,
		consumerID,
		options,
	)
}

// Commit acknowledges a message offset for the given consumer.
func (a *AutoConfigConsumer) Commit(
	ctx context.Context,
	consumerID string,
	offset uint64,
	options *internalapi.MessagePollerv2ConsumerCommitOptions,
) error {
	return a.svix.Message.Pollerv2.ConsumerCommit(
		ctx,
		a.appID,
		a.sinkID,
		consumerID,
		models.PollerV2CommitIn{Offset: offset},
		options,
	)
}
