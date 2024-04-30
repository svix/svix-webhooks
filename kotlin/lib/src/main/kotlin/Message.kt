package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageApi
import com.svix.kotlin.models.ListResponseMessageOut
import com.svix.kotlin.models.MessageIn
import com.svix.kotlin.models.MessageOut

class Message internal constructor(token: String, options: SvixOptions) {
    val api = MessageApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(
        appId: String,
        options: MessageListOptions = MessageListOptions(),
    ): ListResponseMessageOut {
        try {
            return api.v1MessageList(
                appId,
                options.limit,
                options.iterator,
                options.channel,
                options.before,
                options.after,
                options.withContent,
                options.tag,
                HashSet(options.eventTypes),
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(
        appId: String,
        messageIn: MessageIn,
        options: PostOptions = PostOptions(),
    ): MessageOut {
        try {
            return api.v1MessageCreate(appId, messageIn, null, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(
        msgId: String,
        appId: String,
    ): MessageOut {
        try {
            return api.v1MessageGet(appId, msgId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun expungeContent(
        msgId: String,
        appId: String,
    ) {
        try {
            return api.v1MessageExpungeContent(appId, msgId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
