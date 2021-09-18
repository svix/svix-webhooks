package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageApi
import com.svix.kotlin.models.ListResponseMessageOut
import com.svix.kotlin.models.MessageIn
import com.svix.kotlin.models.MessageOut

class MessageListOptions(var eventTypes: List<String> = listOf<String>()) : ListOptions()

class Message internal constructor(debugUrl: String) {
    val api = MessageApi(debugUrl)

    suspend fun list(appId: String, options: MessageListOptions): ListResponseMessageOut {
        try {
            return api.listMessagesApiV1AppAppIdMsgGet(appId, options.iterator, options.limit, options.eventTypes)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun create(appId: String, messageIn: MessageIn): MessageOut {
        try {
            return api.createMessageApiV1AppAppIdMsgPost(appId, messageIn)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun get(msgId: String, appId: String): MessageOut {
        try {
            return api.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }
}
