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
    public sealed class Message : SvixResourceBase, IMessage
    {
        private readonly IMessageApi _messageApi;

        public Message(ISvixClient svixClient, IMessageApi messageApi)
            : base(svixClient)
        {
            _messageApi = messageApi ?? throw new ArgumentException(nameof(messageApi));
        }

        public MessageOut Create(string appId, MessageIn message, MessageCreateOptions options = null, string idempotencyKey = default)
        {
            try
            {
                message = message ?? throw new ArgumentNullException(nameof(message));

                var lApplication = _messageApi.V1MessageCreate(
                    appId,
                    message,
                    options?.WithContent,
                    idempotencyKey);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<MessageOut> CreateAsync(string appId, MessageIn message, MessageCreateOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                message = message ?? throw new ArgumentNullException(nameof(message));

                var lApplication = await _messageApi.V1MessageCreateAsync(
                    appId,
                    message,
                    options?.WithContent,
                    idempotencyKey,
                    cancellationToken);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public MessageOut Get(string appId, string messageId, string idempotencyKey = default)
        {
            try
            {
                var lMessage = _messageApi.V1MessageGet(
                    appId,
                    messageId);

                return lMessage;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<MessageOut> GetAsync(string appId, string messageId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lMessage = await _messageApi.V1MessageGetAsync(
                    appId,
                    messageId);

                return lMessage;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ListResponseMessageOut List(string appId, MessageListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _messageApi.V1MessageList(
                    appId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Channel,
                    options?.Before,
                    options?.After,
                    options?.WithContent,
                    options?.Tag,
                    options?.EventTypes
                    );

                return lResponse;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new ListResponseMessageOut();
            }
        }

        public async Task<ListResponseMessageOut> ListAsync(string appId, MessageListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _messageApi.V1MessageListAsync(
                    appId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Channel,
                    options?.Before,
                    options?.After,
                    options?.WithContent,
                    options?.Tag,
                    options?.EventTypes,
                    cancellationToken);

                return lResponse;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new ListResponseMessageOut();
            }
        }

        public bool ExpungeContent(string appId, string messageId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _messageApi.V1MessageExpungeContentWithHttpInfo(
                    appId,
                    messageId);

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

        public async Task<bool> ExpungeContentAsync(string appId, string messageId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _messageApi.V1MessageExpungeContentWithHttpInfoAsync(
                    appId,
                    messageId,
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
