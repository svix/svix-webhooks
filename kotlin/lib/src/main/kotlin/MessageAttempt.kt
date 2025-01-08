// this file is @generated (with manual changes)
package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageAttemptApi
import com.svix.kotlin.models.ListResponseEndpointMessageOut
import com.svix.kotlin.models.ListResponseMessageAttemptEndpointOut
import com.svix.kotlin.models.ListResponseMessageAttemptOut
import com.svix.kotlin.models.ListResponseMessageEndpointOut
import com.svix.kotlin.models.MessageAttemptOut
import com.svix.kotlin.models.MessageStatus
import com.svix.kotlin.models.StatusCodeClass
import java.time.OffsetDateTime

class MessageAttemptListByEndpointOptions {
    var limit: Int? = null
    var iterator: String? = null
    var status: MessageStatus? = null
    var statusCodeClass: StatusCodeClass? = null
    var channel: String? = null
    var tag: String? = null
    var before: OffsetDateTime? = null
    var after: OffsetDateTime? = null
    var withContent: Boolean? = null
    var withMsg: Boolean? = null
    var eventTypes: List<String>? = null

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }

    /**
     * Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or
     * Sending (3)
     */
    fun status(status: MessageStatus) = apply { this.status = status }

    /** Filter response based on the HTTP status code */
    fun statusCodeClass(statusCodeClass: StatusCodeClass) = apply {
        this.statusCodeClass = statusCodeClass
    }

    /** Filter response based on the channel */
    fun channel(channel: String) = apply { this.channel = channel }

    /** Filter response based on the tag */
    fun tag(tag: String) = apply { this.tag = tag }

    /** Only include items created before a certain date */
    fun before(before: OffsetDateTime) = apply { this.before = before }

    /** Only include items created after a certain date */
    fun after(after: OffsetDateTime) = apply { this.after = after }

    /** When `true` attempt content is included in the response */
    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    /** When `true`, the message information is included in the response */
    fun withMsg(withMsg: Boolean) = apply { this.withMsg = withMsg }

    /** Filter response based on the event type */
    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }
}

class MessageAttemptListByMsgOptions {
    var limit: Int? = null
    var iterator: String? = null
    var status: MessageStatus? = null
    var statusCodeClass: StatusCodeClass? = null
    var channel: String? = null
    var tag: String? = null
    var endpointId: String? = null
    var before: OffsetDateTime? = null
    var after: OffsetDateTime? = null
    var withContent: Boolean? = null
    var eventTypes: List<String>? = null

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }

    /**
     * Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or
     * Sending (3)
     */
    fun status(status: MessageStatus) = apply { this.status = status }

    /** Filter response based on the HTTP status code */
    fun statusCodeClass(statusCodeClass: StatusCodeClass) = apply {
        this.statusCodeClass = statusCodeClass
    }

    /** Filter response based on the channel */
    fun channel(channel: String) = apply { this.channel = channel }

    /** Filter response based on the tag */
    fun tag(tag: String) = apply { this.tag = tag }

    /** Filter the attempts based on the attempted endpoint */
    fun endpointId(endpointId: String) = apply { this.endpointId = endpointId }

    /** Only include items created before a certain date */
    fun before(before: OffsetDateTime) = apply { this.before = before }

    /** Only include items created after a certain date */
    fun after(after: OffsetDateTime) = apply { this.after = after }

    /** When `true` attempt content is included in the response */
    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    /** Filter response based on the event type */
    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }
}

class MessageAttemptListAttemptedMessagesOptions {
    var limit: Int? = null
    var iterator: String? = null
    var channel: String? = null
    var tag: String? = null
    var status: MessageStatus? = null
    var before: OffsetDateTime? = null
    var after: OffsetDateTime? = null
    var withContent: Boolean? = null
    var eventTypes: List<String>? = null

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }

    /** Filter response based on the channel */
    fun channel(channel: String) = apply { this.channel = channel }

    /** Filter response based on the message tags */
    fun tag(tag: String) = apply { this.tag = tag }

    /**
     * Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or
     * Sending (3)
     */
    fun status(status: MessageStatus) = apply { this.status = status }

    /** Only include items created before a certain date */
    fun before(before: OffsetDateTime) = apply { this.before = before }

    /** Only include items created after a certain date */
    fun after(after: OffsetDateTime) = apply { this.after = after }

    /** When `true` message payloads are included in the response */
    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    /** Filter response based on the event type */
    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }
}

class MessageAttemptListAttemptedDestinationsOptions {
    var limit: Int? = null
    var iterator: String? = null

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }
}

class MessageAttemptListOptions(
    var iterator: String? = null,
    var limit: Int? = null,
    var messageStatus: MessageStatus? = null,
    var before: OffsetDateTime? = null,
    var after: OffsetDateTime? = null,
    var eventTypes: List<String>? = null,
    var statusCodeClass: StatusCodeClass? = null,
    var channel: String? = null,
    var tag: String? = null,
    var endpointId: String? = null,
    var withContent: Boolean? = null,
    var withMsg: Boolean? = null,
) {
    fun messageStatus(messageStatus: MessageStatus) = apply { this.messageStatus = messageStatus }

    fun before(before: OffsetDateTime) = apply { this.before = before }

    fun after(after: OffsetDateTime) = apply { this.after = after }

    fun statusCodeClass(statusCodeClass: StatusCodeClass) = apply {
        this.statusCodeClass = statusCodeClass
    }

    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    fun channel(channel: String) = apply { this.channel = channel }

    fun iterator(iterator: String) = apply { this.iterator = iterator }

    fun limit(limit: Int) = apply { this.limit = limit }

    fun endpointId(endpointId: String) = apply { this.endpointId = endpointId }

    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    fun withMsg(withMsg: Boolean) = apply { this.withMsg = withMsg }

    fun tag(tag: String) = apply { this.tag = tag }
}

class MessageAttempt internal constructor(token: String, options: SvixOptions) {
    private val api = MessageAttemptApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** @deprecated use listByMsg or listByEndpoint instead. */
    @Deprecated(message = "use listByMsg or listByEndpoint instead.")
    suspend fun list(
        appId: String,
        msgId: String,
        options: MessageAttemptListByMsgOptions = MessageAttemptListByMsgOptions(),
    ): ListResponseMessageAttemptOut {
        return this.listByMsg(appId, msgId, options)
    }

    /**
     * List attempts by endpoint id
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun listByEndpoint(
        appId: String,
        endpointId: String,
        options: MessageAttemptListByEndpointOptions = MessageAttemptListByEndpointOptions(),
    ): ListResponseMessageAttemptOut {
        try {
            return api.v1MessageAttemptListByEndpoint(
                appId,
                endpointId,
                options.limit,
                options.iterator,
                options.status,
                options.statusCodeClass,
                options.channel,
                options.tag,
                options.before,
                options.after,
                options.withContent,
                options.withMsg,
                HashSet(options.eventTypes),
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * List attempts by message ID.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun listByMsg(
        appId: String,
        msgId: String,
        options: MessageAttemptListByMsgOptions = MessageAttemptListByMsgOptions(),
    ): ListResponseMessageAttemptOut {
        try {
            return api.v1MessageAttemptListByMsg(
                appId,
                msgId,
                options.limit,
                options.iterator,
                options.status,
                options.statusCodeClass,
                options.channel,
                options.tag,
                options.endpointId,
                options.before,
                options.after,
                options.withContent,
                HashSet(options.eventTypes),
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * List messages for a particular endpoint. Additionally includes metadata about the latest
     * message attempt.
     *
     * The `before` parameter lets you filter all items created before a certain date and is ignored
     * if an iterator is passed.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun listAttemptedMessages(
        appId: String,
        endpointId: String,
        options: MessageAttemptListAttemptedMessagesOptions =
            MessageAttemptListAttemptedMessagesOptions(),
    ): ListResponseEndpointMessageOut {
        try {
            return api.v1MessageAttemptListAttemptedMessages(
                appId,
                endpointId,
                options.limit,
                options.iterator,
                options.channel,
                options.tag,
                options.status,
                options.before,
                options.after,
                options.withContent,
                HashSet(options.eventTypes),
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** `msg_id`: Use a message id or a message `eventId` */
    suspend fun get(appId: String, msgId: String, attemptId: String): MessageAttemptOut {
        try {
            return api.v1MessageAttemptGet(appId, msgId, attemptId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Deletes the given attempt's response body.
     *
     * Useful when an endpoint accidentally returned sensitive content. The message can't be
     * replayed or resent once its payload has been deleted or expired.
     */
    suspend fun expungeContent(appId: String, msgId: String, attemptId: String) {
        try {
            api.v1MessageAttemptExpungeContent(appId, msgId, attemptId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * List endpoints attempted by a given message.
     *
     * Additionally includes metadata about the latest message attempt. By default, endpoints are
     * listed in ascending order by ID.
     */
    suspend fun listAttemptedDestinations(
        appId: String,
        msgId: String,
        options: MessageAttemptListAttemptedDestinationsOptions =
            MessageAttemptListAttemptedDestinationsOptions(),
    ): ListResponseMessageEndpointOut {
        try {
            return api.v1MessageAttemptListAttemptedDestinations(
                appId,
                msgId,
                options.limit,
                options.iterator,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    @Deprecated(message = "use listByEndpoint instead.")
    suspend fun listAttemptsForEndpoint(
        appId: String,
        endpointId: String,
        msgId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ): ListResponseMessageAttemptEndpointOut {
        return try {
            api.v1MessageAttemptListByEndpointDeprecated(
                appId,
                msgId,
                endpointId,
                options.limit,
                options.iterator,
                options.channel,
                options.tag,
                options.messageStatus,
                options.before,
                options.after,
                HashSet(options.eventTypes),
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Resend a message to the specified endpoint. */
    suspend fun resend(
        appId: String,
        msgId: String,
        endpointId: String,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1MessageAttemptResend(appId, msgId, endpointId, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
