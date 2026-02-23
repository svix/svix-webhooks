// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class MessageListOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public string? Channel { get; set; }
        public DateTime? Before { get; set; }
        public DateTime? After { get; set; }
        public bool? WithContent { get; set; }
        public string? Tag { get; set; }
        public List<string>? EventTypes { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "channel", Channel },
                    { "before", Before },
                    { "after", After },
                    { "with_content", WithContent },
                    { "tag", Tag },
                    { "event_types", EventTypes },
                }
            );
        }
    }

    public class MessageCreateOptions : SvixOptionsBase
    {
        public bool? WithContent { get; set; }
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "with_content", WithContent } }
            );
        }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class MessageExpungeAllContentsOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class MessagePrecheckOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class MessageGetOptions : SvixOptionsBase
    {
        public bool? WithContent { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "with_content", WithContent } }
            );
        }
    }

    public class Message(SvixClient client)
    {
        readonly SvixClient _client = client;

        public MessagePoller Poller
        {
            get => new MessagePoller(_client);
        }

        /// <summary>Creates a [MessageIn] with a raw string payload.
        /// <para>
        /// The payload is not normalized on the server. Normally, payloads are
        /// required to be JSON, and Svix will minify the payload before sending the
        /// webhooks (for example, by removing extraneous whitespace or unnecessarily
        /// escaped characters in strings). With this function, the payload will be
        /// sent "as is", without any minification or other processing.
        /// </para>
        /// </summary>
        /// <param name="payload">Serialized message payload</param>
        /// <param name="contentType">The `content-type` header of the webhook sent by Svix,
        /// overwriting the default of `application/json` if specified</param>
        public static MessageIn messageInRaw(
            string eventType,
            string payload,
            string? contentType = null,
            ApplicationIn? application = null,
            List<string>? channels = null,
            string? eventId = null,
            long? payloadRetentionHours = null,
            long? payloadRetentionPeriod = null,
            List<string>? tags = null,
            Dictionary<string, Object>? transformationsParams = null
        )
        {
            if (transformationsParams == null)
            {
                transformationsParams = new Dictionary<string, object>();
            }

            transformationsParams["rawPayload"] = payload;
            if (contentType != null)
            {
                transformationsParams["headers"] = new Dictionary<string, string>
                {
                    ["content-type"] = contentType,
                };
            }

            return new MessageIn
            {
                EventType = eventType,
                Payload = new { },
                Application = application,
                Channels = channels,
                EventId = eventId,
                PayloadRetentionHours = payloadRetentionHours,
                PayloadRetentionPeriod = payloadRetentionPeriod,
                Tags = tags,
                TransformationsParams = transformationsParams,
            };
        }

        /// <summary>
        /// List all of the application's messages.
        ///
        /// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be
        /// used alongside an iterator to paginate over results within a certain window.
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
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseMessageOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/app/{app_id}/msg",
                        pathParams: new Dictionary<string, string> { { "app_id", appId } },
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// List all of the application's messages.
        ///
        /// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be
        /// used alongside an iterator to paginate over results within a certain window.
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
                var response = _client.SvixHttpClient.SendRequest<ListResponseMessageOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/msg",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(List)} failed");

                throw;
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
        /// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
        /// </summary>
        public async Task<MessageOut> CreateAsync(
            string appId,
            MessageIn messageIn,
            MessageCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            messageIn = messageIn ?? throw new ArgumentNullException(nameof(messageIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<MessageOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/msg",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: messageIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                throw;
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
        /// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
        /// </summary>
        public MessageOut Create(
            string appId,
            MessageIn messageIn,
            MessageCreateOptions? options = null
        )
        {
            messageIn = messageIn ?? throw new ArgumentNullException(nameof(messageIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<MessageOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/msg",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: messageIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Create)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete all message payloads for the application.
        ///
        /// This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan.
        ///
        /// A completed task will return a payload like the following:
        /// ```json
        /// {
        ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
        ///   "status": "finished",
        ///   "task": "application.purge_content",
        ///   "data": {
        ///     "messagesPurged": 150
        ///   }
        /// }
        /// ```
        /// </summary>
        public async Task<ExpungeAllContentsOut> ExpungeAllContentsAsync(
            string appId,
            MessageExpungeAllContentsOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ExpungeAllContentsOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/msg/expunge-all-contents",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpungeAllContentsAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete all message payloads for the application.
        ///
        /// This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan.
        ///
        /// A completed task will return a payload like the following:
        /// ```json
        /// {
        ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
        ///   "status": "finished",
        ///   "task": "application.purge_content",
        ///   "data": {
        ///     "messagesPurged": 150
        ///   }
        /// }
        /// ```
        /// </summary>
        public ExpungeAllContentsOut ExpungeAllContents(
            string appId,
            MessageExpungeAllContentsOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ExpungeAllContentsOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/msg/expunge-all-contents",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpungeAllContents)} failed");

                throw;
            }
        }

        /// <summary>
        /// A pre-check call for `message.create` that checks whether any active endpoints are
        /// listening to this message.
        ///
        /// Note: most people shouldn't be using this API. Svix doesn't bill you for
        /// messages not actually sent, so using this API doesn't save money.
        /// If unsure, please ask Svix support before using this API.
        /// </summary>
        public async Task<MessagePrecheckOut> PrecheckAsync(
            string appId,
            MessagePrecheckIn messagePrecheckIn,
            MessagePrecheckOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            messagePrecheckIn =
                messagePrecheckIn ?? throw new ArgumentNullException(nameof(messagePrecheckIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<MessagePrecheckOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/msg/precheck/active",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: messagePrecheckIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(PrecheckAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// A pre-check call for `message.create` that checks whether any active endpoints are
        /// listening to this message.
        ///
        /// Note: most people shouldn't be using this API. Svix doesn't bill you for
        /// messages not actually sent, so using this API doesn't save money.
        /// If unsure, please ask Svix support before using this API.
        /// </summary>
        public MessagePrecheckOut Precheck(
            string appId,
            MessagePrecheckIn messagePrecheckIn,
            MessagePrecheckOptions? options = null
        )
        {
            messagePrecheckIn =
                messagePrecheckIn ?? throw new ArgumentNullException(nameof(messagePrecheckIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<MessagePrecheckOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/msg/precheck/active",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: messagePrecheckIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Precheck)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<MessageOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/msg/{msg_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "msg_id", msgId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get a message by its ID or eventID.
        /// </summary>
        public MessageOut Get(string appId, string msgId, MessageGetOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<MessageOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/msg/{msg_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "msg_id", msgId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/msg/{msg_id}/content",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "msg_id", msgId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpungeContentAsync)} failed");

                throw;
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
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/msg/{msg_id}/content",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "msg_id", msgId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpungeContent)} failed");

                throw;
            }
        }
    }
}
