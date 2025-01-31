// this file is @generated
#nullable enable
using System;
using System.Collections.Generic;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;

namespace Svix
{
    public partial class EventTypeListOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
        public bool? IncludeArchived { get; set; }
        public bool? WithContent { get; set; }
    }

    public partial class EventTypeCreateOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EventTypeImportOpenapiOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EventTypeDeleteOptions
    {
        public bool? Expunge { get; set; }
    }

    public sealed class EventType : SvixResourceBase
    {
        private readonly EventTypeApi _eventTypeApi;

        public EventType(ISvixClient svixClient, EventTypeApi eventTypeApi)
            : base(svixClient)
        {
            _eventTypeApi = eventTypeApi ?? throw new ArgumentNullException(nameof(eventTypeApi));
        }

        /// <summary>
        /// Return the list of event types.
        /// </summary>
        public async Task<ListResponseEventTypeOut> ListAsync(
            EventTypeListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _eventTypeApi.V1EventTypeListWithHttpInfoAsync(
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order,
                    includeArchived: options?.IncludeArchived,
                    withContent: options?.WithContent,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseEventTypeOut();
            }
        }

        /// <summary>
        /// Return the list of event types.
        /// </summary>
        public ListResponseEventTypeOut List(EventTypeListOptions? options = null)
        {
            try
            {
                var response = _eventTypeApi.V1EventTypeListWithHttpInfo(
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order,
                    includeArchived: options?.IncludeArchived,
                    withContent: options?.WithContent
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;
                return new ListResponseEventTypeOut();
            }
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
            try
            {
                eventTypeIn = eventTypeIn ?? throw new ArgumentNullException(nameof(eventTypeIn));
                var response = await _eventTypeApi.V1EventTypeCreateWithHttpInfoAsync(
                    eventTypeIn: eventTypeIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
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
            try
            {
                eventTypeIn = eventTypeIn ?? throw new ArgumentNullException(nameof(eventTypeIn));
                var response = _eventTypeApi.V1EventTypeCreateWithHttpInfo(
                    eventTypeIn: eventTypeIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
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
            try
            {
                eventTypeImportOpenApiIn =
                    eventTypeImportOpenApiIn
                    ?? throw new ArgumentNullException(nameof(eventTypeImportOpenApiIn));
                var response = await _eventTypeApi.V1EventTypeImportOpenapiWithHttpInfoAsync(
                    eventTypeImportOpenApiIn: eventTypeImportOpenApiIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ImportOpenapiAsync)} failed");

                if (Throw)
                    throw;
                return new EventTypeImportOpenApiOut();
            }
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
            try
            {
                eventTypeImportOpenApiIn =
                    eventTypeImportOpenApiIn
                    ?? throw new ArgumentNullException(nameof(eventTypeImportOpenApiIn));
                var response = _eventTypeApi.V1EventTypeImportOpenapiWithHttpInfo(
                    eventTypeImportOpenApiIn: eventTypeImportOpenApiIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ImportOpenapi)} failed");

                if (Throw)
                    throw;
                return new EventTypeImportOpenApiOut();
            }
        }

        /// <summary>
        /// Get an event type.
        /// </summary>
        public async Task<EventTypeOut> GetAsync(
            string eventTypeName,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _eventTypeApi.V1EventTypeGetWithHttpInfoAsync(
                    eventTypeName: eventTypeName,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
        }

        /// <summary>
        /// Get an event type.
        /// </summary>
        public EventTypeOut Get(string eventTypeName)
        {
            try
            {
                var response = _eventTypeApi.V1EventTypeGetWithHttpInfo(
                    eventTypeName: eventTypeName
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
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
            try
            {
                eventTypeUpdate =
                    eventTypeUpdate ?? throw new ArgumentNullException(nameof(eventTypeUpdate));
                var response = await _eventTypeApi.V1EventTypeUpdateWithHttpInfoAsync(
                    eventTypeName: eventTypeName,
                    eventTypeUpdate: eventTypeUpdate,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
        }

        /// <summary>
        /// Update an event type.
        /// </summary>
        public EventTypeOut Update(string eventTypeName, EventTypeUpdate eventTypeUpdate)
        {
            try
            {
                eventTypeUpdate =
                    eventTypeUpdate ?? throw new ArgumentNullException(nameof(eventTypeUpdate));
                var response = _eventTypeApi.V1EventTypeUpdateWithHttpInfo(
                    eventTypeName: eventTypeName,
                    eventTypeUpdate: eventTypeUpdate
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
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
            try
            {
                var response = await _eventTypeApi.V1EventTypeDeleteWithHttpInfoAsync(
                    eventTypeName: eventTypeName,
                    expunge: options?.Expunge,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                var response = _eventTypeApi.V1EventTypeDeleteWithHttpInfo(
                    eventTypeName: eventTypeName,
                    expunge: options?.Expunge
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                eventTypePatch =
                    eventTypePatch ?? throw new ArgumentNullException(nameof(eventTypePatch));
                var response = await _eventTypeApi.V1EventTypePatchWithHttpInfoAsync(
                    eventTypeName: eventTypeName,
                    eventTypePatch: eventTypePatch,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
        }

        /// <summary>
        /// Partially update an event type.
        /// </summary>
        public EventTypeOut Patch(string eventTypeName, EventTypePatch eventTypePatch)
        {
            try
            {
                eventTypePatch =
                    eventTypePatch ?? throw new ArgumentNullException(nameof(eventTypePatch));
                var response = _eventTypeApi.V1EventTypePatchWithHttpInfo(
                    eventTypeName: eventTypeName,
                    eventTypePatch: eventTypePatch
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Patch)} failed");

                if (Throw)
                    throw;
                return new EventTypeOut();
            }
        }
    }
}
