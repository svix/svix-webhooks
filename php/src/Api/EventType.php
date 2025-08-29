<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\EventTypeImportOpenApiIn;
use Svix\Models\EventTypeImportOpenApiOut;
use Svix\Models\EventTypeIn;
use Svix\Models\EventTypeOut;
use Svix\Models\EventTypePatch;
use Svix\Models\EventTypeUpdate;
use Svix\Models\ListResponseEventTypeOut;
use Svix\Request\SvixHttpClient;

class EventType
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** Return the list of event types. */
    public function list(
        ?EventTypeListOptions $options = null,
    ): ListResponseEventTypeOut {
        $request = $this->client->newReq('GET', '/api/v1/event-type');
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
            $request->setQueryParam('include_archived', $options->includeArchived);
            $request->setQueryParam('with_content', $options->withContent);
        }
        $res = $this->client->send($request);

        return ListResponseEventTypeOut::fromJson($res);
    }

    /**
     * Create new or unarchive existing event type.
     *
     * Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
     * Endpoints filtering on the event type before archival will continue to filter on it.
     * This operation does not preserve the description and schemas.
     */
    public function create(
        EventTypeIn $eventTypeIn,
        ?EventTypeCreateOptions $options = null,
    ): EventTypeOut {
        $request = $this->client->newReq('POST', '/api/v1/event-type');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($eventTypeIn));
        $res = $this->client->send($request);

        return EventTypeOut::fromJson($res);
    }

    /**
     * Given an OpenAPI spec, create new or update existing event types.
     * If an existing `archived` event type is updated, it will be unarchived.
     *
     * The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
     * top-level.
     */
    public function importOpenapi(
        EventTypeImportOpenApiIn $eventTypeImportOpenApiIn,
        ?EventTypeImportOpenapiOptions $options = null,
    ): EventTypeImportOpenApiOut {
        $request = $this->client->newReq('POST', '/api/v1/event-type/import/openapi');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($eventTypeImportOpenApiIn));
        $res = $this->client->send($request);

        return EventTypeImportOpenApiOut::fromJson($res);
    }

    /** Get an event type. */
    public function get(
        string $eventTypeName,
    ): EventTypeOut {
        $request = $this->client->newReq('GET', "/api/v1/event-type/{$eventTypeName}");
        $res = $this->client->send($request);

        return EventTypeOut::fromJson($res);
    }

    /** Update an event type. */
    public function update(
        string $eventTypeName,
        EventTypeUpdate $eventTypeUpdate,
    ): EventTypeOut {
        $request = $this->client->newReq('PUT', "/api/v1/event-type/{$eventTypeName}");
        $request->setBody(json_encode($eventTypeUpdate));
        $res = $this->client->send($request);

        return EventTypeOut::fromJson($res);
    }

    /**
     * Archive an event type.
     *
     * Endpoints already configured to filter on an event type will continue to do so after archival.
     * However, new messages can not be sent with it and endpoints can not filter on it.
     * An event type can be unarchived with the
     * [create operation](#operation/create_event_type_api_v1_event_type__post).
     */
    public function delete(
        string $eventTypeName,
        ?EventTypeDeleteOptions $options = null,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/event-type/{$eventTypeName}");
        if (null !== $options) {
            $request->setQueryParam('expunge', $options->expunge);
        }
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially update an event type. */
    public function patch(
        string $eventTypeName,
        EventTypePatch $eventTypePatch,
    ): EventTypeOut {
        $request = $this->client->newReq('PATCH', "/api/v1/event-type/{$eventTypeName}");
        $request->setBody(json_encode($eventTypePatch));
        $res = $this->client->send($request);

        return EventTypeOut::fromJson($res);
    }
}
