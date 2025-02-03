// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class EventTypeListOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
        public bool? IncludeArchived { get; set; }
        public bool? WithContent { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                    { "include_archived", IncludeArchived },
                    { "with_content", WithContent },
                }
            );
        }
    }

    public class EventTypeCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EventTypeImportOpenapiOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EventTypeDeleteOptions : SvixOptionsBase
    {
        public bool? Expunge { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(new Dictionary<string, object?> { { "expunge", Expunge } });
        }
    }

    public class EventType(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Return the list of event types.
        /// </summary>
        public async Task<ListResponseEventTypeOut> ListAsync(
            EventTypeListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseEventTypeOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/event-type",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Return the list of event types.
        /// </summary>
        public ListResponseEventTypeOut List(EventTypeListOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseEventTypeOut>(
                method: HttpMethod.Get,
                path: "/api/v1/event-type",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// Create new or unarchive existing event type.
        ///
        /// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
        /// Endpoints filtering on the event type before archival will continue to filter on it.
        /// This operation does not preserve the description and schemas.
        /// </summary>
        public async Task<EventTypeOut> CreateAsync(
            EventTypeIn eventTypeIn,
            EventTypeCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            eventTypeIn = eventTypeIn ?? throw new ArgumentNullException(nameof(eventTypeIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<EventTypeOut>(
                method: HttpMethod.Post,
                path: "/api/v1/event-type",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: eventTypeIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Create new or unarchive existing event type.
        ///
        /// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
        /// Endpoints filtering on the event type before archival will continue to filter on it.
        /// This operation does not preserve the description and schemas.
        /// </summary>
        public EventTypeOut Create(EventTypeIn eventTypeIn, EventTypeCreateOptions? options = null)
        {
            eventTypeIn = eventTypeIn ?? throw new ArgumentNullException(nameof(eventTypeIn));

            var response = this._client.SvixHttpClient.SendRequest<EventTypeOut>(
                method: HttpMethod.Post,
                path: "/api/v1/event-type",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: eventTypeIn
            );
            return response.Data;
        }

        /// <summary>
        /// Given an OpenAPI spec, create new or update existing event types.
        /// If an existing `archived` event type is updated, it will be unarchived.
        ///
        /// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
        /// top-level.
        /// </summary>
        public async Task<EventTypeImportOpenApiOut> ImportOpenapiAsync(
            EventTypeImportOpenApiIn eventTypeImportOpenApiIn,
            EventTypeImportOpenapiOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            eventTypeImportOpenApiIn =
                eventTypeImportOpenApiIn
                ?? throw new ArgumentNullException(nameof(eventTypeImportOpenApiIn));

            var response =
                await this._client.SvixHttpClient.SendRequestAsync<EventTypeImportOpenApiOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/event-type/import/openapi",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: eventTypeImportOpenApiIn,
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Given an OpenAPI spec, create new or update existing event types.
        /// If an existing `archived` event type is updated, it will be unarchived.
        ///
        /// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
        /// top-level.
        /// </summary>
        public EventTypeImportOpenApiOut ImportOpenapi(
            EventTypeImportOpenApiIn eventTypeImportOpenApiIn,
            EventTypeImportOpenapiOptions? options = null
        )
        {
            eventTypeImportOpenApiIn =
                eventTypeImportOpenApiIn
                ?? throw new ArgumentNullException(nameof(eventTypeImportOpenApiIn));

            var response = this._client.SvixHttpClient.SendRequest<EventTypeImportOpenApiOut>(
                method: HttpMethod.Post,
                path: "/api/v1/event-type/import/openapi",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: eventTypeImportOpenApiIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get an event type.
        /// </summary>
        public async Task<EventTypeOut> GetAsync(
            string eventTypeName,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<EventTypeOut>(
                method: HttpMethod.Get,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get an event type.
        /// </summary>
        public EventTypeOut Get(string eventTypeName)
        {
            var response = this._client.SvixHttpClient.SendRequest<EventTypeOut>(
                method: HttpMethod.Get,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } }
            );
            return response.Data;
        }

        /// <summary>
        /// Update an event type.
        /// </summary>
        public async Task<EventTypeOut> UpdateAsync(
            string eventTypeName,
            EventTypeUpdate eventTypeUpdate,
            CancellationToken cancellationToken = default
        )
        {
            eventTypeUpdate =
                eventTypeUpdate ?? throw new ArgumentNullException(nameof(eventTypeUpdate));

            var response = await this._client.SvixHttpClient.SendRequestAsync<EventTypeOut>(
                method: HttpMethod.Put,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                content: eventTypeUpdate,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Update an event type.
        /// </summary>
        public EventTypeOut Update(string eventTypeName, EventTypeUpdate eventTypeUpdate)
        {
            eventTypeUpdate =
                eventTypeUpdate ?? throw new ArgumentNullException(nameof(eventTypeUpdate));

            var response = this._client.SvixHttpClient.SendRequest<EventTypeOut>(
                method: HttpMethod.Put,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                content: eventTypeUpdate
            );
            return response.Data;
        }

        /// <summary>
        /// Archive an event type.
        ///
        /// Endpoints already configured to filter on an event type will continue to do so after archival.
        /// However, new messages can not be sent with it and endpoints can not filter on it.
        /// An event type can be unarchived with the
        /// [create operation](#operation/create_event_type_api_v1_event_type__post).
        /// </summary>
        public async Task<bool> DeleteAsync(
            string eventTypeName,
            EventTypeDeleteOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Archive an event type.
        ///
        /// Endpoints already configured to filter on an event type will continue to do so after archival.
        /// However, new messages can not be sent with it and endpoints can not filter on it.
        /// An event type can be unarchived with the
        /// [create operation](#operation/create_event_type_api_v1_event_type__post).
        /// </summary>
        public bool Delete(string eventTypeName, EventTypeDeleteOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// Partially update an event type.
        /// </summary>
        public async Task<EventTypeOut> PatchAsync(
            string eventTypeName,
            EventTypePatch eventTypePatch,
            CancellationToken cancellationToken = default
        )
        {
            eventTypePatch =
                eventTypePatch ?? throw new ArgumentNullException(nameof(eventTypePatch));

            var response = await this._client.SvixHttpClient.SendRequestAsync<EventTypeOut>(
                method: HttpMethod.Patch,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                content: eventTypePatch,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Partially update an event type.
        /// </summary>
        public EventTypeOut Patch(string eventTypeName, EventTypePatch eventTypePatch)
        {
            eventTypePatch =
                eventTypePatch ?? throw new ArgumentNullException(nameof(eventTypePatch));

            var response = this._client.SvixHttpClient.SendRequest<EventTypeOut>(
                method: HttpMethod.Patch,
                path: "/api/v1/event-type/{event_type_name}",
                pathParams: new Dictionary<string, string> { { "event_type_name", eventTypeName } },
                content: eventTypePatch
            );
            return response.Data;
        }
    }
}
