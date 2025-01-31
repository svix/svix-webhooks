// this file is @generated
#nullable enable
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
    public partial class MessageListOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public string? Channel { get; set; }
        public DateTime? Before { get; set; }
        public DateTime? After { get; set; }
        public bool? WithContent { get; set; }
        public string? Tag { get; set; }
        public List<string>? EventTypes { get; set; }
    }

    public partial class MessageCreateOptions
    {
        public bool? WithContent { get; set; }
        public string? IdempotencyKey { get; set; }
    }

    public partial class MessageGetOptions
    {
        public bool? WithContent { get; set; }
    }

    public sealed class Message : SvixResourceBase
    {
        private readonly MessageApi _messageApi;

        public Message(ISvixClient svixClient, MessageApi messageApi)
            : base(svixClient)
        {
            _messageApi = messageApi ?? throw new ArgumentNullException(nameof(messageApi));
        }

        /// <summary>
        /// List all of the application's messages.
        ///
        /// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
        /// within a certain window.
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public async Task<ListResponseMessageOut> ListAsync(
            string appId,
            MessageListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _messageApi.V1MessageListWithHttpInfoAsync(
                    appId: appId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    channel: options?.Channel,
                    before: options?.Before,
                    after: options?.After,
                    withContent: options?.WithContent,
                    tag: options?.Tag,
                    eventTypes: options?.EventTypes,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageOut();
            }
        }

        /// <summary>
        /// List all of the application's messages.
        ///
        /// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
        /// within a certain window.
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public ListResponseMessageOut List(string appId, MessageListOptions? options = null)
        {
            try
            {
                var response = _messageApi.V1MessageListWithHttpInfo(
                    appId: appId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    channel: options?.Channel,
                    before: options?.Before,
                    after: options?.After,
                    withContent: options?.WithContent,
                    tag: options?.Tag,
                    eventTypes: options?.EventTypes
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageOut();
            }
        }

        /// <summary>
        /// Creates a new message and dispatches it to all of the application's endpoints.
        ///
        /// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
        /// If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
        ///
        /// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
        /// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
        ///
        /// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
        /// </summary>
        public async Task<MessageOut> CreateAsync(
            string appId,
            MessageIn messageIn,
            MessageCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                messageIn = messageIn ?? throw new ArgumentNullException(nameof(messageIn));
                var response = await _messageApi.V1MessageCreateWithHttpInfoAsync(
                    appId: appId,
                    messageIn: messageIn,
                    withContent: options?.WithContent,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new MessageOut();
            }
        }

        /// <summary>
        /// Creates a new message and dispatches it to all of the application's endpoints.
        ///
        /// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
        /// If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
        ///
        /// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
        /// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
        ///
        /// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
        /// </summary>
        public MessageOut Create(
            string appId,
            MessageIn messageIn,
            MessageCreateOptions? options = null
        )
        {
            try
            {
                messageIn = messageIn ?? throw new ArgumentNullException(nameof(messageIn));
                var response = _messageApi.V1MessageCreateWithHttpInfo(
                    appId: appId,
                    messageIn: messageIn,
                    withContent: options?.WithContent,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new MessageOut();
            }
        }

        /// <summary>
        /// Get a message by its ID or eventID.
        /// </summary>
        public async Task<MessageOut> GetAsync(
            string appId,
            string msgId,
            MessageGetOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _messageApi.V1MessageGetWithHttpInfoAsync(
                    appId: appId,
                    msgId: msgId,
                    withContent: options?.WithContent,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new MessageOut();
            }
        }

        /// <summary>
        /// Get a message by its ID or eventID.
        /// </summary>
        public MessageOut Get(string appId, string msgId, MessageGetOptions? options = null)
        {
            try
            {
                var response = _messageApi.V1MessageGetWithHttpInfo(
                    appId: appId,
                    msgId: msgId,
                    withContent: options?.WithContent
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new MessageOut();
            }
        }

        /// <summary>
        /// Delete the given message's payload.
        ///
        /// Useful in cases when a message was accidentally sent with sensitive content.
        /// The message can't be replayed or resent once its payload has been deleted or expired.
        /// </summary>
        public async Task<bool> ExpungeContentAsync(
            string appId,
            string msgId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _messageApi.V1MessageExpungeContentWithHttpInfoAsync(
                    appId: appId,
                    msgId: msgId,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ExpungeContentAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Delete the given message's payload.
        ///
        /// Useful in cases when a message was accidentally sent with sensitive content.
        /// The message can't be replayed or resent once its payload has been deleted or expired.
        /// </summary>
        public bool ExpungeContent(string appId, string msgId)
        {
            try
            {
                var response = _messageApi.V1MessageExpungeContentWithHttpInfo(
                    appId: appId,
                    msgId: msgId
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ExpungeContent)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }
    }
}
