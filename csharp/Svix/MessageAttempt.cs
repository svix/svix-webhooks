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
    public partial class MessageAttemptListByEndpointOptions
    {
        public int? Limit { get; set; }
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
    }

    public partial class MessageAttemptListByMsgOptions
    {
        public int? Limit { get; set; }
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
    }

    public partial class MessageAttemptListAttemptedMessagesOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public string? Channel { get; set; }
        public string? Tag { get; set; }
        public MessageStatus? Status { get; set; }
        public DateTime? Before { get; set; }
        public DateTime? After { get; set; }
        public bool? WithContent { get; set; }
        public List<string>? EventTypes { get; set; }
    }

    public partial class MessageAttemptListAttemptedDestinationsOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
    }

    public partial class MessageAttemptResendOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public sealed class MessageAttempt : SvixResourceBase
    {
        private readonly MessageAttemptApi _messageAttemptApi;

        public MessageAttempt(ISvixClient svixClient, MessageAttemptApi messageAttemptApi)
            : base(svixClient)
        {
            _messageAttemptApi =
                messageAttemptApi ?? throw new ArgumentNullException(nameof(messageAttemptApi));
        }

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
            try
            {
                var response =
                    await _messageAttemptApi.V1MessageAttemptListByEndpointWithHttpInfoAsync(
                        appId: appId,
                        endpointId: endpointId,
                        limit: options?.Limit,
                        iterator: options?.Iterator,
                        status: options?.Status,
                        statusCodeClass: options?.StatusCodeClass,
                        channel: options?.Channel,
                        tag: options?.Tag,
                        before: options?.Before,
                        after: options?.After,
                        withContent: options?.WithContent,
                        withMsg: options?.WithMsg,
                        eventTypes: options?.EventTypes,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListByEndpointAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageAttemptOut();
            }
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
            try
            {
                var response = _messageAttemptApi.V1MessageAttemptListByEndpointWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    status: options?.Status,
                    statusCodeClass: options?.StatusCodeClass,
                    channel: options?.Channel,
                    tag: options?.Tag,
                    before: options?.Before,
                    after: options?.After,
                    withContent: options?.WithContent,
                    withMsg: options?.WithMsg,
                    eventTypes: options?.EventTypes
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListByEndpoint)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageAttemptOut();
            }
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
            try
            {
                var response = await _messageAttemptApi.V1MessageAttemptListByMsgWithHttpInfoAsync(
                    appId: appId,
                    msgId: msgId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    status: options?.Status,
                    statusCodeClass: options?.StatusCodeClass,
                    channel: options?.Channel,
                    tag: options?.Tag,
                    endpointId: options?.EndpointId,
                    before: options?.Before,
                    after: options?.After,
                    withContent: options?.WithContent,
                    eventTypes: options?.EventTypes,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListByMsgAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageAttemptOut();
            }
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
            try
            {
                var response = _messageAttemptApi.V1MessageAttemptListByMsgWithHttpInfo(
                    appId: appId,
                    msgId: msgId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    status: options?.Status,
                    statusCodeClass: options?.StatusCodeClass,
                    channel: options?.Channel,
                    tag: options?.Tag,
                    endpointId: options?.EndpointId,
                    before: options?.Before,
                    after: options?.After,
                    withContent: options?.WithContent,
                    eventTypes: options?.EventTypes
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListByMsg)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageAttemptOut();
            }
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
            try
            {
                var response =
                    await _messageAttemptApi.V1MessageAttemptListAttemptedMessagesWithHttpInfoAsync(
                        appId: appId,
                        endpointId: endpointId,
                        limit: options?.Limit,
                        iterator: options?.Iterator,
                        channel: options?.Channel,
                        tag: options?.Tag,
                        status: options?.Status,
                        before: options?.Before,
                        after: options?.After,
                        withContent: options?.WithContent,
                        eventTypes: options?.EventTypes,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedMessagesAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseEndpointMessageOut();
            }
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
            try
            {
                var response = _messageAttemptApi.V1MessageAttemptListAttemptedMessagesWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    channel: options?.Channel,
                    tag: options?.Tag,
                    status: options?.Status,
                    before: options?.Before,
                    after: options?.After,
                    withContent: options?.WithContent,
                    eventTypes: options?.EventTypes
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedMessages)} failed");

                if (Throw)
                    throw;
                return new ListResponseEndpointMessageOut();
            }
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
            try
            {
                var response = await _messageAttemptApi.V1MessageAttemptGetWithHttpInfoAsync(
                    appId: appId,
                    msgId: msgId,
                    attemptId: attemptId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new MessageAttemptOut();
            }
        }

        /// <summary>
        /// `msg_id`: Use a message id or a message `eventId`
        /// </summary>
        public MessageAttemptOut Get(string appId, string msgId, string attemptId)
        {
            try
            {
                var response = _messageAttemptApi.V1MessageAttemptGetWithHttpInfo(
                    appId: appId,
                    msgId: msgId,
                    attemptId: attemptId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new MessageAttemptOut();
            }
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
            try
            {
                var response =
                    await _messageAttemptApi.V1MessageAttemptExpungeContentWithHttpInfoAsync(
                        appId: appId,
                        msgId: msgId,
                        attemptId: attemptId,
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
        /// Deletes the given attempt's response body.
        ///
        /// Useful when an endpoint accidentally returned sensitive content.
        /// The message can't be replayed or resent once its payload has been deleted or expired.
        /// </summary>
        public bool ExpungeContent(string appId, string msgId, string attemptId)
        {
            try
            {
                var response = _messageAttemptApi.V1MessageAttemptExpungeContentWithHttpInfo(
                    appId: appId,
                    msgId: msgId,
                    attemptId: attemptId
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
            try
            {
                var response =
                    await _messageAttemptApi.V1MessageAttemptListAttemptedDestinationsWithHttpInfoAsync(
                        appId: appId,
                        msgId: msgId,
                        limit: options?.Limit,
                        iterator: options?.Iterator,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedDestinationsAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageEndpointOut();
            }
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
            try
            {
                var response =
                    _messageAttemptApi.V1MessageAttemptListAttemptedDestinationsWithHttpInfo(
                        appId: appId,
                        msgId: msgId,
                        limit: options?.Limit,
                        iterator: options?.Iterator
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAttemptedDestinations)} failed");

                if (Throw)
                    throw;
                return new ListResponseMessageEndpointOut();
            }
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
            try
            {
                var response = await _messageAttemptApi.V1MessageAttemptResendWithHttpInfoAsync(
                    appId: appId,
                    msgId: msgId,
                    endpointId: endpointId,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ResendAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                var response = _messageAttemptApi.V1MessageAttemptResendWithHttpInfo(
                    appId: appId,
                    msgId: msgId,
                    endpointId: endpointId,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Resend)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }
    }
}
