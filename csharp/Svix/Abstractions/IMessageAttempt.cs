using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IMessageAttempt
    {
        List<EndpointMessageOut> ListAttemptedMessages(string appId, string endpointId, MessageAttemptListOptions options, 
            string idempotencyKey = default);

        Task<List<EndpointMessageOut>> ListAttemptedMessagesAsync(string appId, string endpointId, MessageAttemptListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        List<MessageAttemptEndpointOut> ListAttemptsByEndpoint(string appId, string endpointId,
            AttemptsByEndpointListOptions options, string idempotencyKey = default);
        
        Task<List<MessageAttemptEndpointOut>> ListAttemptsByEndpointAsync(string appId, string endpointId,
            AttemptsByEndpointListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
        
        List<MessageAttemptOut> ListAttemptsByMessage(string appId, string messageId,
            MessageAttemptByMessageListOptions options, string idempotencyKey = default);
        
        Task<List<MessageAttemptOut>> ListAttemptsByMessageAsync(string appId, string messageId,
            MessageAttemptByMessageListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
        
        List<MessageEndpointOut> ListAttemptedDestinations(string appId, string messageId,
            ListOptions options, string idempotencyKey = default);
        
        Task<List<MessageEndpointOut>> ListAttemptedDestinationsAsync(string appId, string messageId,
            ListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool ResendWebhook(string appId, string messageId, string endpointId, string idempotencyKey = default);

        Task<bool> ResendWebhookAsync(string appId, string messageId, string endpointId,
            string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}