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
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ): ListResponseMessageAttemptOut {
        return this.listByMsg(appId, msgId, options)
    }

    suspend fun listByMsg(
        appId: String,
        msgId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ): ListResponseMessageAttemptOut {
        try {
            return api.v1MessageAttemptListByMsg(
                appId,
                msgId,
                options.limit,
                options.iterator,
                options.messageStatus,
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

    suspend fun listByEndpoint(
        appId: String,
        endpointId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ): ListResponseMessageAttemptOut {
        try {
            return api.v1MessageAttemptListByEndpoint(
                appId,
                endpointId,
                options.limit,
                options.iterator,
                options.messageStatus,
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

    suspend fun get(appId: String, msgId: String, attemptId: String): MessageAttemptOut {
        try {
            return api.v1MessageAttemptGet(appId, msgId, attemptId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

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

    suspend fun listAttemptedMessages(
        appId: String,
        endpointId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ): ListResponseEndpointMessageOut {
        try {
            return api.v1MessageAttemptListAttemptedMessages(
                appId,
                endpointId,
                options.limit,
                options.iterator,
                options.channel,
                options.tag,
                options.messageStatus,
                options.before,
                options.after,
                options.withContent,
                HashSet(options.eventTypes),
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun listAttemptedDestinations(
        appId: String,
        msgId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
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

    suspend fun expungeContent(appId: String, msgId: String, attemptId: String) {
        try {
            return api.v1MessageAttemptExpungeContent(appId, msgId, attemptId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
