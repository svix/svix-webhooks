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

        public bool Archive(string eventType, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _eventTypeApi.DeleteEventTypeApiV1EventTypeEventTypeNameDeleteWithHttpInfo(
                    eventType,
                    null,
                    idempotencyKey);

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

        public async Task<bool> ArchiveAsync(string eventType, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _eventTypeApi.DeleteEventTypeApiV1EventTypeEventTypeNameDeleteWithHttpInfoAsync(
                    eventType,
                    null,
                    idempotencyKey,
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
                var lEventType = _eventTypeApi.CreateEventTypeApiV1EventTypePost(
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
                var lEventType = await _eventTypeApi.CreateEventTypeApiV1EventTypePostAsync(
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
                var lEventType = _eventTypeApi.GetEventTypeApiV1EventTypeEventTypeNameGet(
                    eventType,
                    idempotencyKey);

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
                var lEventType = await _eventTypeApi.GetEventTypeApiV1EventTypeEventTypeNameGetAsync(
                    eventType,
                    idempotencyKey,
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

        public List<EventTypeOut> List(EventTypeListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResults = _eventTypeApi.ListEventTypesApiV1EventTypeGet(
                    options?.Iterator,
                    options?.Limit,
                    options?.WithContent,
                    options?.IncludeArchived,
                    idempotencyKey);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new List<EventTypeOut>();
            }
        }

        public async Task<List<EventTypeOut>> ListAsync(EventTypeListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _eventTypeApi.ListEventTypesApiV1EventTypeGetAsync(
                    options?.Iterator,
                    options?.Limit,
                    options?.WithContent,
                    options?.IncludeArchived,
                    idempotencyKey,
                    cancellationToken);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new List<EventTypeOut>();
            }
        }

        public EventTypeOut Update(string eventType, EventTypeUpdate update, string idempotencyKey = default)
        {
            try
            {
                var lEventType = _eventTypeApi.UpdateEventTypeApiV1EventTypeEventTypeNamePut(
                    eventType,
                    update,
                    idempotencyKey);

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
                var lEventType = await _eventTypeApi.UpdateEventTypeApiV1EventTypeEventTypeNamePutAsync(
                    eventType,
                    update,
                    idempotencyKey,
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
    }
}
