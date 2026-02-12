// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.EventTypeImportOpenApiIn;
import com.svix.models.EventTypeImportOpenApiOut;
import com.svix.models.EventTypeIn;
import com.svix.models.EventTypeOut;
import com.svix.models.EventTypePatch;
import com.svix.models.EventTypeUpdate;
import com.svix.models.ListResponseEventTypeOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class EventType {
    private final SvixHttpClient client;

    public EventType(SvixHttpClient client) {
        this.client = client;
    }

    /** Return the list of event types. */
    public ListResponseEventTypeOut list() throws IOException, ApiException {

        return this.list(new EventTypeListOptions());
    }

    /** Return the list of event types. */
    public ListResponseEventTypeOut list(final EventTypeListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/event-type");
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.order != null) {
            url.addQueryParameter("order", Utils.serializeQueryParam(options.order));
        }
        if (options.includeArchived != null) {
            url.addQueryParameter(
                    "include_archived", Utils.serializeQueryParam(options.includeArchived));
        }
        if (options.withContent != null) {
            url.addQueryParameter("with_content", Utils.serializeQueryParam(options.withContent));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseEventTypeOut.class);
    }

    /**
     * Create new or unarchive existing event type.
     *
     * <p>Unarchiving an event type will allow endpoints to filter on it and messages to be sent
     * with it. Endpoints filtering on the event type before archival will continue to filter on it.
     * This operation does not preserve the description and schemas.
     */
    public EventTypeOut create(final EventTypeIn eventTypeIn) throws IOException, ApiException {
        return this.create(eventTypeIn, new EventTypeCreateOptions());
    }

    /**
     * Create new or unarchive existing event type.
     *
     * <p>Unarchiving an event type will allow endpoints to filter on it and messages to be sent
     * with it. Endpoints filtering on the event type before archival will continue to filter on it.
     * This operation does not preserve the description and schemas.
     */
    public EventTypeOut create(final EventTypeIn eventTypeIn, final EventTypeCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/event-type");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), eventTypeIn, EventTypeOut.class);
    }

    /**
     * Given an OpenAPI spec, create new or update existing event types.
     *
     * <p>If an existing `archived` event type is updated, it will be unarchived. The importer will
     * convert all webhooks found in the either the `webhooks` or `x-webhooks` top-level.
     */
    public EventTypeImportOpenApiOut importOpenapi(
            final EventTypeImportOpenApiIn eventTypeImportOpenApiIn)
            throws IOException, ApiException {
        return this.importOpenapi(eventTypeImportOpenApiIn, new EventTypeImportOpenapiOptions());
    }

    /**
     * Given an OpenAPI spec, create new or update existing event types.
     *
     * <p>If an existing `archived` event type is updated, it will be unarchived. The importer will
     * convert all webhooks found in the either the `webhooks` or `x-webhooks` top-level.
     */
    public EventTypeImportOpenApiOut importOpenapi(
            final EventTypeImportOpenApiIn eventTypeImportOpenApiIn,
            final EventTypeImportOpenapiOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath("/api/v1/event-type/import/openapi");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                eventTypeImportOpenApiIn,
                EventTypeImportOpenApiOut.class);
    }

    /** Get an event type. */
    public EventTypeOut get(final String eventTypeName) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/event-type/%s", eventTypeName));
        return this.client.executeRequest("GET", url.build(), null, null, EventTypeOut.class);
    }

    /** Update an event type. */
    public EventTypeOut update(final String eventTypeName, final EventTypeUpdate eventTypeUpdate)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/event-type/%s", eventTypeName));
        return this.client.executeRequest(
                "PUT", url.build(), null, eventTypeUpdate, EventTypeOut.class);
    }

    /**
     * Archive an event type.
     *
     * <p>Endpoints already configured to filter on an event type will continue to do so after
     * archival. However, new messages can not be sent with it and endpoints can not filter on it.
     * An event type can be unarchived with the [create
     * operation](#operation/create_event_type_api_v1_event_type__post).
     */
    public void delete(final String eventTypeName) throws IOException, ApiException {
        this.delete(eventTypeName, new EventTypeDeleteOptions());
    }

    /**
     * Archive an event type.
     *
     * <p>Endpoints already configured to filter on an event type will continue to do so after
     * archival. However, new messages can not be sent with it and endpoints can not filter on it.
     * An event type can be unarchived with the [create
     * operation](#operation/create_event_type_api_v1_event_type__post).
     */
    public void delete(final String eventTypeName, final EventTypeDeleteOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/event-type/%s", eventTypeName));
        if (options.expunge != null) {
            url.addQueryParameter("expunge", Utils.serializeQueryParam(options.expunge));
        }
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update an event type. */
    public EventTypeOut patch(final String eventTypeName, final EventTypePatch eventTypePatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/event-type/%s", eventTypeName));
        return this.client.executeRequest(
                "PATCH", url.build(), null, eventTypePatch, EventTypeOut.class);
    }
}
