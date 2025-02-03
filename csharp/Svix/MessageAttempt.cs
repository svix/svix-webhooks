// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class MessageAttemptListByEndpointOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public MessageStatus? Status { get; set; }
        public StatusCodeClass? StatusCodeClass { get; set; }
        public string? Channel { get; set; }
        public string? Tag { get; set; }
        public DateTime? Before { get; set; }
        public DateTime? After { get; set; }
        public bool? WithContent { get; set; }
        public bool? WithMsg { get; set; }
        public List<string>? EventTypes { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "status", Status },
                    { "status_code_class", StatusCodeClass },
                    { "channel", Channel },
                    { "tag", Tag },
                    { "before", Before },
                    { "after", After },
                    { "with_content", WithContent },
                    { "with_msg", WithMsg },
                    { "event_types", EventTypes },
                }
            );
        }
    }

    public class MessageAttemptListByMsgOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public MessageStatus? Status { get; set; }
        public StatusCodeClass? StatusCodeClass { get; set; }
        public string? Channel { get; set; }
        public string? Tag { get; set; }
        public string? EndpointId { get; set; }
        public DateTime? Before { get; set; }
        public DateTime? After { get; set; }
        public bool? WithContent { get; set; }
        public List<string>? EventTypes { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "status", Status },
                    { "status_code_class", StatusCodeClass },
                    { "channel", Channel },
                    { "tag", Tag },
                    { "endpoint_id", EndpointId },
                    { "before", Before },
                    { "after", After },
                    { "with_content", WithContent },
                    { "event_types", EventTypes },
                }
            );
        }
    }

    public class MessageAttemptListAttemptedMessagesOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public string? Channel { get; set; }
        public string? Tag { get; set; }
        public MessageStatus? Status { get; set; }
        public DateTime? Before { get; set; }
        public DateTime? After { get; set; }
        public bool? WithContent { get; set; }
        public List<string>? EventTypes { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "channel", Channel },
                    { "tag", Tag },
                    { "status", Status },
                    { "before", Before },
                    { "after", After },
                    { "with_content", WithContent },
                    { "event_types", EventTypes },
                }
            );
        }
    }

    public class MessageAttemptListAttemptedDestinationsOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "limit", Limit }, { "iterator", Iterator } }
            );
        }
    }

    public class MessageAttemptResendOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class MessageAttempt(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List attempts by endpoint id
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public async Task<ListResponseMessageAttemptOut> ListByEndpointAsync(
            string appId,
            string endpointId,
            MessageAttemptListByEndpointOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseMessageAttemptOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List attempts by endpoint id
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public ListResponseMessageAttemptOut ListByEndpoint(
            string appId,
            string endpointId,
            MessageAttemptListByEndpointOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseMessageAttemptOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// List attempts by message ID.
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public async Task<ListResponseMessageAttemptOut> ListByMsgAsync(
            string appId,
            string msgId,
            MessageAttemptListByMsgOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseMessageAttemptOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/attempt/msg/{msg_id}",
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

        /// <summary>
        /// List attempts by message ID.
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public ListResponseMessageAttemptOut ListByMsg(
            string appId,
            string msgId,
            MessageAttemptListByMsgOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseMessageAttemptOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/attempt/msg/{msg_id}",
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

        /// <summary>
        /// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
        ///
        /// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public async Task<ListResponseEndpointMessageOut> ListAttemptedMessagesAsync(
            string appId,
            string endpointId,
            MessageAttemptListAttemptedMessagesOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseEndpointMessageOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
        ///
        /// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
        ///
        /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
        /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
        /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        /// set the `before` or `after` parameter as appropriate.
        /// </summary>
        public ListResponseEndpointMessageOut ListAttemptedMessages(
            string appId,
            string endpointId,
            MessageAttemptListAttemptedMessagesOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseEndpointMessageOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// `msg_id`: Use a message id or a message `eventId`
        /// </summary>
        public async Task<MessageAttemptOut> GetAsync(
            string appId,
            string msgId,
            string attemptId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<MessageAttemptOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "msg_id", msgId },
                    { "attempt_id", attemptId },
                },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// `msg_id`: Use a message id or a message `eventId`
        /// </summary>
        public MessageAttemptOut Get(string appId, string msgId, string attemptId)
        {
            var response = this._client.SvixHttpClient.SendRequest<MessageAttemptOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "msg_id", msgId },
                    { "attempt_id", attemptId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// Deletes the given attempt's response body.
        ///
        /// Useful when an endpoint accidentally returned sensitive content.
        /// The message can't be replayed or resent once its payload has been deleted or expired.
        /// </summary>
        public async Task<bool> ExpungeContentAsync(
            string appId,
            string msgId,
            string attemptId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "msg_id", msgId },
                    { "attempt_id", attemptId },
                },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Deletes the given attempt's response body.
        ///
        /// Useful when an endpoint accidentally returned sensitive content.
        /// The message can't be replayed or resent once its payload has been deleted or expired.
        /// </summary>
        public bool ExpungeContent(string appId, string msgId, string attemptId)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "msg_id", msgId },
                    { "attempt_id", attemptId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// List endpoints attempted by a given message.
        ///
        /// Additionally includes metadata about the latest message attempt.
        /// By default, endpoints are listed in ascending order by ID.
        /// </summary>
        public async Task<ListResponseMessageEndpointOut> ListAttemptedDestinationsAsync(
            string appId,
            string msgId,
            MessageAttemptListAttemptedDestinationsOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseMessageEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
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

        /// <summary>
        /// List endpoints attempted by a given message.
        ///
        /// Additionally includes metadata about the latest message attempt.
        /// By default, endpoints are listed in ascending order by ID.
        /// </summary>
        public ListResponseMessageEndpointOut ListAttemptedDestinations(
            string appId,
            string msgId,
            MessageAttemptListAttemptedDestinationsOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseMessageEndpointOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
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

        /// <summary>
        /// Resend a message to the specified endpoint.
        /// </summary>
        public async Task<bool> ResendAsync(
            string appId,
            string msgId,
            string endpointId,
            MessageAttemptResendOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "msg_id", msgId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Resend a message to the specified endpoint.
        /// </summary>
        public bool Resend(
            string appId,
            string msgId,
            string endpointId,
            MessageAttemptResendOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "msg_id", msgId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }
    }
}
