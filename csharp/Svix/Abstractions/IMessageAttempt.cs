using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IMessageAttempt
    {
        MessageAttemptOut GetAttempt(string appId, string attemptId, string messageId, string idempotencyKey = default);

        Task<MessageAttemptOut> GetAttemptAsync(string appId, string attemptId, string messageId, string idempotencyKey = default, CancellationToken cancellationToken = default);

        ListResponseEndpointMessageOut ListAttemptedMessages(string appId, string endpointId, MessageAttemptListOptions options = null,
            string idempotencyKey = default);

        Task<ListResponseEndpointMessageOut> ListAttemptedMessagesAsync(string appId, string endpointId, MessageAttemptListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        ListResponseMessageAttemptOut ListAttemptsByEndpoint(string appId, string endpointId, AttemptsByEndpointListOptions options = null, string idempotencyKey = default);

        Task<ListResponseMessageAttemptOut> ListAttemptsByEndpointAsync(string appId, string endpointId, AttemptsByEndpointListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);

        ListResponseMessageAttemptOut ListAttemptsByMessage(string appId, string messageId, AttemptsByMessageListOptions options = null, string idempotencyKey = default);

        Task<ListResponseMessageAttemptOut> ListAttemptsByMessageAsync(string appId, string messageId, AttemptsByMessageListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);

        ListResponseMessageAttemptEndpointOut ListAttemptsForEndpoint(string appId, string messageId, string endpointId,
            AttemptsByEndpointListOptions options = null, string idempotencyKey = default);

        Task<ListResponseMessageAttemptEndpointOut> ListAttemptsForEndpointAsync(string appId, string messageId, string endpointId,
            AttemptsByEndpointListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ListResponseMessageAttemptOut ListAttempts(string appId, string messageId,
            MessageAttemptListOptions options = null, string idempotencyKey = default);

        Task<ListResponseMessageAttemptOut> ListAttemptsAsync(string appId, string messageId,
            MessageAttemptListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ListResponseMessageEndpointOut ListAttemptedDestinations(string appId, string messageId,
            ListOptions options = null, string idempotencyKey = default);

        Task<ListResponseMessageEndpointOut> ListAttemptedDestinationsAsync(string appId, string messageId,
            ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool ResendWebhook(string appId, string messageId, string endpointId, string idempotencyKey = default);

        Task<bool> ResendWebhookAsync(string appId, string messageId, string endpointId,
            string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}
