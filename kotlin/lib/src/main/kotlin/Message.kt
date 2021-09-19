package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageApi
import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.models.ListResponseMessageOut
import com.svix.kotlin.models.MessageIn
import com.svix.kotlin.models.MessageOut

class MessageListOptions(var eventTypes: List<String> = listOf<String>()) : ListOptions() {
    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    override fun iterator(iterator: kotlin.String) = apply { super.iterator(iterator) }

    override fun limit(limit: kotlin.Int) = apply { super.limit(limit) }
}

class Message internal constructor(debugUrl: String) {
    val api = MessageApi(debugUrl)

    suspend fun list(appId: String, options: MessageListOptions): ListResponseMessageOut {
        try {
            return api.listMessagesApiV1AppAppIdMsgGet(appId, options.iterator, options.limit, options.eventTypes)
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun create(appId: String, messageIn: MessageIn): MessageOut {
        try {
            return api.createMessageApiV1AppAppIdMsgPost(appId, messageIn)
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun get(msgId: String, appId: String): MessageOut {
        try {
            return api.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId)
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }
}
