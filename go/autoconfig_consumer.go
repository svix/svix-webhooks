package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internalapi"
	"github.com/svix/svix-webhooks/go/models"
)

// AutoConfigConsumer decodes an auto-configuration token for polling sink consumers.
type AutoConfigConsumer struct {
	appID        string
	sinkID       string
	autoconfigID string
	sinkIn       models.SinkInCommon
	svix         *internalapi.InternalSvix
}

// NewAutoConfigConsumer parses an auto-config token and prepares consumer helpers.
func NewAutoConfigConsumer(token string, sinkIn models.SinkInCommon) (*AutoConfigConsumer, error) {
	decoded, err := decodeAutoConfigToken(token)
	if err != nil {
		return nil, err
	}

	svx, err := newInternalSvixFromAutoConfig(decoded.serverURL, decoded.tokenPlaintext)
	if err != nil {
		return nil, err
	}

	consumer := &AutoConfigConsumer{
		appID:  decoded.appID,
		sinkIn: sinkIn,
		svix:   svx,
	}
	if decoded.version == autoConfigTokenVersionV1 {
		consumer.sinkID = decoded.endpointID
	} else {
		consumer.autoconfigID = decoded.autoconfigID
	}
	return consumer, nil
}

func sinkInCommonToPollingDestination(sink models.SinkInCommon) models.DestinationIn {
	return models.DestinationIn{
		Uid:        sink.Uid,
		EventTypes: sink.EventTypes,
		Channels:   sink.Channels,
		Metadata:   sink.Metadata,
		Type:       models.DestinationInTypePollingEndpoint,
	}
}

// Subscribe registers or updates the polling sink via the auto-config API.
func (a *AutoConfigConsumer) Subscribe(ctx context.Context) (*models.DestinationOut, error) {
	if a.autoconfigID != "" {
		dest, err := a.svix.Destination().Autoconfig().Subscribe(
			ctx,
			a.appID,
			a.autoconfigID,
			sinkInCommonToPollingDestination(a.sinkIn),
		)
		if err != nil {
			return nil, err
		}
		a.sinkID = dest.Id
		return dest, nil
	}

	endpoint, err := a.svix.Endpoint().AutoConfigDeprecated().Update(
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
	if err != nil {
		return nil, err
	}
	return destinationOutFromV1Endpoint(endpoint), nil
}

func destinationOutFromV1Endpoint(endpoint *models.EndpointOut) *models.DestinationOut {
	status := models.SINKSTATUS_ENABLED
	if endpoint.Disabled != nil && *endpoint.Disabled {
		status = models.SINKSTATUS_DISABLED
	}
	return &models.DestinationOut{
		Id:         endpoint.Id,
		Uid:        endpoint.Uid,
		Status:     status,
		CreatedAt:  endpoint.CreatedAt,
		UpdatedAt:  endpoint.UpdatedAt,
		EventTypes: endpoint.EventTypes,
		Channels:   endpoint.Channels,
		Metadata:   endpoint.Metadata,
		Type:       models.DestinationOutTypePollingEndpoint,
	}
}

func (a *AutoConfigConsumer) ensureSinkID(ctx context.Context) error {
	if a.sinkID != "" {
		return nil
	}
	dest, err := a.Subscribe(ctx)
	if err != nil {
		return err
	}
	a.sinkID = dest.Id
	return nil
}

// Receive polls messages from the sink for the given consumer.
func (a *AutoConfigConsumer) Receive(
	ctx context.Context,
	consumerID string,
	options *internalapi.MessagePollerv2ConsumerPollOptions,
) (*models.PollerV2PollOut, error) {
	if err := a.ensureSinkID(ctx); err != nil {
		return nil, err
	}
	return a.svix.Message().Pollerv2().ConsumerPoll(
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
	if err := a.ensureSinkID(ctx); err != nil {
		return err
	}
	return a.svix.Message().Pollerv2().ConsumerCommit(
		ctx,
		a.appID,
		a.sinkID,
		consumerID,
		models.PollerV2CommitIn{Offset: offset},
		options,
	)
}
