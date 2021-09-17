package com.svix.kotlin

import com.svix.kotlin.internal.apis.MessageApi
import com.svix.kotlin.internal.models.MessageIn
import com.svix.kotlin.internal.models.MessageOut
import com.svix.kotlin.internal.models.ListResponseMessageOut
import com.svix.kotlin.internal.infrastructure.ServerException

class Message() {
	val api = MessageApi()

	suspend fun list(appId: String, options: MessageListOptions): ListResponseMessageOut {
		try {
			return api.listMessagesApiV1AppAppIdMsgGet(appId, options.iterator, options.limit, options.eventTypes)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun create(appId: String, messageIn: MessageIn): MessageOut {
		try {
			return api.createMessageApiV1AppAppIdMsgPost(appId, messageIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun get(msgId: String, appId: String): MessageOut {
		try {
			return api.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}
}
