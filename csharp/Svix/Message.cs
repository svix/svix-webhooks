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

                var lApplication = _messageApi.CreateMessageApiV1AppAppIdMsgPost(
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

                var lApplication = await _messageApi.CreateMessageApiV1AppAppIdMsgPostAsync(
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
                var lMessage = _messageApi.GetMessageApiV1AppAppIdMsgMsgIdGet(
                    messageId,
                    appId,
                    idempotencyKey);

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
                var lMessage = await _messageApi.GetMessageApiV1AppAppIdMsgMsgIdGetAsync(
                    messageId,
                    appId,
                    idempotencyKey);

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

        public List<MessageOut> List(string appId, MessageListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _messageApi.ListMessagesApiV1AppAppIdMsgGet(
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    options?.EventTypes,
                    options?.Channel,
                    options?.Before,
                    options?.After,
                    idempotencyKey);

                return lResponse?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new List<MessageOut>();
            }
        }

        public async Task<List<MessageOut>> ListAsync(string appId, MessageListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _messageApi.ListMessagesApiV1AppAppIdMsgGetAsync(
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    options?.EventTypes,
                    options?.Channel,
                    options?.Before,
                    options?.After,
                    idempotencyKey,
                    cancellationToken);

                return lResponse?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new List<MessageOut>();
            }
        }

        public bool ExpungeContent(string appId, string messageId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _messageApi.ExpungeMessagePayloadApiV1AppAppIdMsgMsgIdContentDeleteWithHttpInfo(
                    messageId,
                    appId,
                    idempotencyKey);

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
                var lResponse = await _messageApi.ExpungeMessagePayloadApiV1AppAppIdMsgMsgIdContentDeleteWithHttpInfoAsync(
                    messageId,
                    appId,
                    idempotencyKey,
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
