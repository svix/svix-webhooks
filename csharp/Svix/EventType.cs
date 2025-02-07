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
    public sealed class EventType : SvixResourceBase, IEventType
    {
        private readonly IEventTypeApi _eventTypeApi;

        public EventType(ISvixClient svixClient, IEventTypeApi eventTypeApi)
            : base(svixClient)
        {
            _eventTypeApi = eventTypeApi ?? throw new ArgumentNullException(nameof(eventTypeApi));
        }

        public bool Archive(string eventType, bool? expunge = null, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _eventTypeApi.V1EventTypeDeleteWithHttpInfo(
                    eventType,
                    expunge);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Archive)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> ArchiveAsync(string eventType, bool? expunge = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _eventTypeApi.V1EventTypeDeleteWithHttpInfoAsync(
                    eventType,
                    expunge,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ArchiveAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public EventTypeOut Create(EventTypeIn eventType, string idempotencyKey = default)
        {
            try
            {
                var lEventType = _eventTypeApi.V1EventTypeCreate(
                    eventType,
                    idempotencyKey);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EventTypeOut> CreateAsync(EventTypeIn eventType, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lEventType = await _eventTypeApi.V1EventTypeCreateAsync(
                    eventType,
                    idempotencyKey,
                    cancellationToken);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public EventTypeOut Get(string eventType, string idempotencyKey = default)
        {
            try
            {
                var lEventType = _eventTypeApi.V1EventTypeGet(eventType);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EventTypeOut> GetAsync(string eventType, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lEventType = await _eventTypeApi.V1EventTypeGetAsync(
                    eventType,
                    cancellationToken);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ListResponseEventTypeOut List(EventTypeListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResults = _eventTypeApi.V1EventTypeList(
                    options?.Limit,
                    options?.Iterator,
                    null,
                    options?.IncludeArchived,
                    options?.WithContent);

                return lResults;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new ListResponseEventTypeOut();
            }
        }

        public async Task<ListResponseEventTypeOut> ListAsync(EventTypeListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _eventTypeApi.V1EventTypeListAsync(
                    options?.Limit,
                    options?.Iterator,
                    null,
                    options?.IncludeArchived,
                    options?.WithContent,
                    cancellationToken);

                return lResults;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new ListResponseEventTypeOut();
            }
        }

        public EventTypeOut Update(string eventType, EventTypeUpdate update, string idempotencyKey = default)
        {
            try
            {
                var lEventType = _eventTypeApi.V1EventTypeUpdate(
                    eventType,
                    update);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EventTypeOut> UpdateAsync(string eventType, EventTypeUpdate update, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEventType = await _eventTypeApi.V1EventTypeUpdateAsync(
                    eventType,
                    update,
                    cancellationToken);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public EventTypeOut Patch(string eventType, EventTypePatch update, string idempotencyKey = default)
        {
            try
            {
                var lEventType = _eventTypeApi.V1EventTypePatch(
                    eventType,
                    update);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Patch)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EventTypeOut> PatchAsync(string eventType, EventTypePatch update, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEventType = await _eventTypeApi.V1EventTypePatchAsync(
                    eventType,
                    update,
                    cancellationToken);

                return lEventType;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public EventTypeImportOpenApiOut ImportOpenApi(EventTypeImportOpenApiIn eventTypeImportOpenApiIn, string idempotencyKey = default)
        {
            try
            {
                var lEventTypeImportOpenApiOut = _eventTypeApi.V1EventTypeImportOpenapi(
                    eventTypeImportOpenApiIn,
                    idempotencyKey);

                return lEventTypeImportOpenApiOut;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EventTypeImportOpenApiOut> ImportOpenApiAsync(EventTypeImportOpenApiIn eventTypeImportOpenApiIn, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lEventTypeImportOpenApiOut = await _eventTypeApi.V1EventTypeImportOpenapiAsync(
                    eventTypeImportOpenApiIn,
                    idempotencyKey,
                    cancellationToken);

                return lEventTypeImportOpenApiOut;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }
    }
}
