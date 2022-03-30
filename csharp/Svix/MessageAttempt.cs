using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Abstractions;
using Svix.Api;
using Svix.Model;
using Svix.Models;

namespace Svix
{
    public sealed class MessageAttempt : SvixResourceBase, IMessageAttempt
    {
        private readonly IMessageAttemptApi _messageAttemptApi;

        public MessageAttempt(ISvixClient svixClient) : base(svixClient)
        {
        }

        public List<MessageAttemptOut> ListAttemptedMessages(string appId, string endpointId, MessageAttemptListOptions options,
            string idempotencyKey = default)
        {
            throw new NotImplementedException();
        }

        public Task<List<MessageAttemptOut>> ListAttemptedMessagesAsync(string appId, string endpointId, MessageAttemptListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }

        public List<MessageAttemptOut> ListAttemptedMessagesByEndpoint(string appId, string endpointId, MessageAttemptByEndpointListOptions options,
            string idempotencyKey = default)
        {
            throw new NotImplementedException();
        }

        public Task<List<MessageAttemptOut>> ListAttemptedMessagesByEndpointAsync(string appId, string endpointId, MessageAttemptByEndpointListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }

        public List<MessageAttemptOut> ListAttemptedMessagesByMessage(string appId, string messageId, MessageAttemptByMessageListOptions options,
            string idempotencyKey = default)
        {
            throw new NotImplementedException();
        }

        public Task<List<MessageAttemptOut>> ListAttemptedMessagesByMessageAsync(string appId, string messageId, MessageAttemptByMessageListOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }

        public List<MessageAttemptOut> ListAttemptedMessagesByDestination(string appId, string messageId, MessageAttemptByDestinationListOptions options,
            string idempotencyKey = default)
        {
            throw new NotImplementedException();
        }

        public Task<List<MessageAttemptOut>> ListAttemptedMessagesByDestinationAsync(string appId, string messageId,
            MessageAttemptByDestinationListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }

        public bool ResendWebhook(string appId, string messageId, string endpointId, string idempotencyKey = default)
        {
            throw new NotImplementedException();
        }

        public Task<bool> ResendWebhookAsync(string appId, string messageId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }
    }
}