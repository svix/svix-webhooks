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

        List<EndpointMessageOut> ListAttemptedMessages(string appId, string endpointId, MessageAttemptListOptions options = null,
            string idempotencyKey = default);

        Task<List<EndpointMessageOut>> ListAttemptedMessagesAsync(string appId, string endpointId, MessageAttemptListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        List<MessageAttemptOut> ListAttemptsByEndpoint(string appId, string endpointId, AttemptsByEndpointListOptions options = null, string idempotencyKey = default);

        Task<List<MessageAttemptOut>> ListAttemptsByEndpointAsync(string appId, string endpointId, AttemptsByEndpointListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);

        List<MessageAttemptOut> ListAttemptsByMessage(string appId, string messageId, AttemptsByMessageListOptions options = null, string idempotencyKey = default);

        Task<List<MessageAttemptOut>> ListAttemptsByMessageAsync(string appId, string messageId, AttemptsByMessageListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);

        List<MessageAttemptEndpointOut> ListAttemptsForEndpoint(string appId, string messageId, string endpointId,
            AttemptsByEndpointListOptions options = null, string idempotencyKey = default);

        Task<List<MessageAttemptEndpointOut>> ListAttemptsForEndpointAsync(string appId, string messageId, string endpointId,
            AttemptsByEndpointListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        List<MessageAttemptOut> ListAttempts(string appId, string messageId,
            MessageAttemptListOptions options = null, string idempotencyKey = default);

        Task<List<MessageAttemptOut>> ListAttemptsAsync(string appId, string messageId,
            MessageAttemptListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        List<MessageEndpointOut> ListAttemptedDestinations(string appId, string messageId,
            ListOptions options = null, string idempotencyKey = default);

        Task<List<MessageEndpointOut>> ListAttemptedDestinationsAsync(string appId, string messageId,
            ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool ResendWebhook(string appId, string messageId, string endpointId, string idempotencyKey = default);

        Task<bool> ResendWebhookAsync(string appId, string messageId, string endpointId,
            string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}
