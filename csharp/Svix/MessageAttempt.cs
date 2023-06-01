using System;
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

        public MessageAttemptOut GetAttempt(string appId, string attemptId, string messageId, string idempotencyKey = default)
        {
            try
            {
                var lAttempt = _messageAttemptApi.V1MessageAttemptGet(
                    appId,
                    messageId,
                    attemptId);

                return lAttempt;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAttempt)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<MessageAttemptOut> GetAttemptAsync(string appId, string attemptId, string messageId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lAttempt = await _messageAttemptApi.V1MessageAttemptGetAsync(
                    appId,
                    messageId,
                    attemptId,
                    cancellationToken);

                return lAttempt;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAttemptAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public List<EndpointMessageOut> ListAttemptedMessages(string appId, string endpointId, MessageAttemptListOptions options = null,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.V1MessageAttemptListAttemptedMessages(
                    appId,
                    endpointId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Channel,
                    (Svix.Model.MessageStatus?)options?.Status,
                    options?.Before,
                    options?.After);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedMessages)} failed");

                if (Throw)
                    throw;

                return new List<EndpointMessageOut>();
            }
        }

        public async Task<List<EndpointMessageOut>> ListAttemptedMessagesAsync(string appId, string endpointId, MessageAttemptListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.V1MessageAttemptListAttemptedMessagesAsync(
                    appId,
                    endpointId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Channel,
                    (Svix.Model.MessageStatus?)options?.Status,
                    options?.Before,
                    options?.After,
                    options?.WithContent,
                    cancellationToken);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedMessagesAsync)} failed");

                if (Throw)
                    throw;

                return new List<EndpointMessageOut>();
            }
        }

        public List<MessageAttemptOut> ListAttemptsByEndpoint(string appId, string endpointId, AttemptsByEndpointListOptions options = null,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.V1MessageAttemptListByEndpoint(
                    appId,
                    endpointId,
                    options?.Limit,
                    options?.Iterator,
                    (Svix.Model.MessageStatus?)options?.Status,
                    (Svix.Model.StatusCodeClass?)options?.Code,
                    options?.Channel,
                    options?.Before,
                    options?.After,
                    options?.EventTypes);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByEndpoint)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        public async Task<List<MessageAttemptOut>> ListAttemptsByEndpointAsync(string appId, string endpointId, AttemptsByEndpointListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.V1MessageAttemptListByEndpointAsync(
                    appId,
                    endpointId,
                    options?.Limit,
                    options?.Iterator,
                    (Svix.Model.MessageStatus?)options?.Status,
                    (Svix.Model.StatusCodeClass?)options?.Code,
                    options?.Channel,
                    options?.Before,
                    options?.After,
                    options?.EventTypes,
                    cancellationToken);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByEndpointAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        public List<MessageAttemptOut> ListAttemptsByMessage(string appId, string messageId, AttemptsByMessageListOptions options = null,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.V1MessageAttemptListByMsg(
                    appId,
                    messageId,
                    options?.Limit,
                    options?.Iterator,
                    (Svix.Model.MessageStatus?)options?.Status,
                    (Svix.Model.StatusCodeClass?)options?.Code,
                    options?.Channel,
                    options?.EndpointId,
                    options?.Before,
                    options?.After,
                    options?.EventTypes);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByMessage)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        public async Task<List<MessageAttemptOut>> ListAttemptsByMessageAsync(string appId, string messageId, AttemptsByMessageListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.V1MessageAttemptListByMsgAsync(
                    appId,
                    messageId,
                    options?.Limit,
                    options?.Iterator,
                    (Svix.Model.MessageStatus?)options?.Status,
                    (Svix.Model.StatusCodeClass?)options?.Code,
                    options?.Channel,
                    options?.EndpointId,
                    options?.Before,
                    options?.After,
                    options?.EventTypes,
                    cancellationToken);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsByMessageAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        // Deprecated
        public List<MessageAttemptEndpointOut> ListAttemptsForEndpoint(string appId, string messageId,
            string endpointId, AttemptsByEndpointListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.V1MessageAttemptListByEndpointDeprecated(
                    appId,
                    messageId,
                    endpointId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Channel,
                    (Svix.Model.MessageStatus?)options?.Status,
                    options?.Before,
                    options?.After,
                    options?.EventTypes);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsForEndpoint)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptEndpointOut>();
            }
        }

        // Deprecated
        public async Task<List<MessageAttemptEndpointOut>> ListAttemptsForEndpointAsync(string appId,
            string messageId, string endpointId, AttemptsByEndpointListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.V1MessageAttemptListByEndpointDeprecatedAsync(
                    appId,
                    messageId,
                    endpointId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Channel,
                    (Svix.Model.MessageStatus?)options?.Status,
                    options?.Before,
                    options?.After,
                    options?.EventTypes?.ToList(),
                    cancellationToken);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsForEndpointAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptEndpointOut>();
            }
        }

        // Deprecated
        public List<MessageAttemptOut> ListAttempts(string appId, string messageId, MessageAttemptListOptions options = null,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.V1MessageAttemptListByMsgDeprecated(
                    appId,
                    messageId,
                    options?.Limit,
                    options?.Iterator,
                    options?.EndpointId,
                    options?.Channel,
                    (Svix.Model.MessageStatus?)options?.Status,
                    options?.Before,
                    options?.After,
                    null,
                    options?.EventTypes);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttempts)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        // Deprecated
        public async Task<List<MessageAttemptOut>> ListAttemptsAsync(string appId, string messageId, MessageAttemptListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.V1MessageAttemptListByMsgDeprecatedAsync(
                    appId,
                    messageId,
                    options?.Limit,
                    options?.Iterator,
                    options?.EndpointId,
                    options?.Channel,
                    (Svix.Model.MessageStatus?)options?.Status,
                    options?.Before,
                    options?.After,
                    null,
                    options?.EventTypes,
                    cancellationToken);

                return lResults?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptsAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageAttemptOut>();
            }
        }

        public List<MessageEndpointOut> ListAttemptedDestinations(string appId, string messageId, ListOptions options = null,
            string idempotencyKey = default)
        {
            try
            {
                var lResults = _messageAttemptApi.V1MessageAttemptListAttemptedDestinations(
                    appId,
                    messageId,
                    options?.Limit,
                    options?.Iterator);

                return lResults?.Data;
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
            ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResults = await _messageAttemptApi.V1MessageAttemptListAttemptedDestinationsAsync(
                    appId,
                    messageId,
                    options?.Limit,
                    options?.Iterator,
                    cancellationToken);

                return lResults?.Data;
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
                var lResponse = _messageAttemptApi.V1MessageAttemptResendWithHttpInfo(
                    appId,
                    messageId,
                    endpointId,
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
                var lResponse = await _messageAttemptApi.V1MessageAttemptResendWithHttpInfoAsync(
                    appId,
                    messageId,
                    endpointId,
                    idempotencyKey,
                    cancellationToken);

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

        public bool ExpungeContent(string appId, string messageId, string attemptId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _messageAttemptApi.V1MessageAttemptExpungeContentWithHttpInfo(
                    appId,
                    messageId,
                    attemptId);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ExpungeContent)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> ExpungeContentAsync(string appId, string messageId, string attemptId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _messageAttemptApi.V1MessageAttemptExpungeContentWithHttpInfoAsync(
                    appId,
                    messageId,
                    attemptId,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ExpungeContentAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }
    }
}
