﻿using System;
using System.Collections.Generic;
using System.Linq;
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
    public sealed class MessageAttempt : SvixResourceBase, IMessageAttempt
    {
        private readonly IMessageAttemptApi _messageAttemptApi;

        public MessageAttempt(ISvixClient svixClient, IMessageAttemptApi messageAttemptApi) 
            : base(svixClient)
        {
            _messageAttemptApi = messageAttemptApi ?? throw new ArgumentNullException(nameof(messageAttemptApi));
        }

        public List<EndpointMessageOut> ListAttemptedMessages(string appId, string endpointId, MessageAttemptListOptions options,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(
                    endpointId,
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    (Svix.Model.MessageStatus)options?.Status,
                    options?.Before,
                    idempotencyKey);

                return lResults.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedMessages)} failed");

                if (Throw)
                    throw;

                return new List<EndpointMessageOut>();
            }
        }

        public async Task<List<EndpointMessageOut>> ListAttemptedMessagesAsync(string appId, string endpointId, MessageAttemptListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGetAsync(
                    endpointId,
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    (Svix.Model.MessageStatus)options?.Status,
                    options?.Before,
                    idempotencyKey,
                    cancellationToken);

                return lResults.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedMessagesAsync)} failed");

                if (Throw)
                    throw;

                return new List<EndpointMessageOut>();
            }
        }

        public List<MessageAttemptEndpointOut> ListAttemptsByEndpoint(string appId, string endpointId, AttemptsByEndpointListOptions options,
            string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByEndpoint)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptEndpointOut>();
            }
        }

        public async Task<List<MessageAttemptEndpointOut>> ListAttemptsByEndpointAsync(string appId, string endpointId, AttemptsByEndpointListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByEndpointAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptEndpointOut>();
            }
        }

        public List<MessageAttemptOut> ListAttemptsByMessage(string appId, string messageId, MessageAttemptByMessageListOptions options,
            string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByMessage)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        public async Task<List<MessageAttemptOut>> ListAttemptsByMessageAsync(string appId, string messageId, MessageAttemptByMessageListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByMessageAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        public List<MessageEndpointOut> ListAttemptedDestinations(string appId, string messageId, ListOptions options,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(
                    messageId,
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    idempotencyKey);

                return lResults.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedDestinations)} failed");

                if (Throw)
                    throw;

                return new List<MessageEndpointOut>();
            }
        }

        public async Task<List<MessageEndpointOut>> ListAttemptedDestinationsAsync(string appId, string messageId,
            ListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGetAsync(
                    messageId,
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    idempotencyKey,
                    cancellationToken);

                return lResults.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedDestinationsAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageEndpointOut>();
            }
        }

        public bool ResendWebhook(string appId, string messageId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _messageAttemptApi.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostWithHttpInfo(
                    endpointId,
                    messageId,
                    appId,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.Accepted;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ResendWebhook)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> ResendWebhookAsync(string appId, string messageId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _messageAttemptApi.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostWithHttpInfoAsync(
                    endpointId,
                    messageId,
                    appId,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.Accepted;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ResendWebhookAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }
    }
}