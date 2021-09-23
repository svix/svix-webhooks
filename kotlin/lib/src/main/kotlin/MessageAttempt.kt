package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageAttemptApi
import com.svix.kotlin.models.ListResponseEndpointMessageOut
import com.svix.kotlin.models.ListResponseMessageAttemptEndpointOut
import com.svix.kotlin.models.ListResponseMessageAttemptOut
import com.svix.kotlin.models.ListResponseMessageEndpointOut
import com.svix.kotlin.models.MessageAttemptOut

class MessageAttempt internal constructor(token: String, options: SvixOptions) {
    val api = MessageAttemptApi(options.debugUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun list(appId: String, msgId: String, options: MessageAttemptListOptions = MessageAttemptListOptions()): ListResponseMessageAttemptOut {
        try {
            return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(
                appId,
                msgId,
                options.iterator,
                options.limit,
                options.messageStatus
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String, msgId: String, attemptId: String): MessageAttemptOut {
        try {
            return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun resend(appId: String, msgId: String, endpointId: String) {
        try {
            api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun listAttemptedMessages(
        appId: String,
        endpointId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions()
    ): ListResponseEndpointMessageOut {
        try {
            return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(
                endpointId,
                appId,
                options.iterator,
                options.limit,
                options.messageStatus,
                options.before
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun listAttemptedDestinations(
        appId: String,
        msgId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions()
    ): ListResponseMessageEndpointOut {
        try {
            return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(
                msgId,
                appId,
                options.iterator,
                options.limit
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun listAttemptsForEndpoint(
        appId: String,
        endpointId: String,
        msgId: String,
        options: MessageAttemptListOptions = MessageAttemptListOptions()
    ): ListResponseMessageAttemptEndpointOut {
        return try {
            api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(
                msgId,
                appId,
                endpointId,
                options.iterator,
                options.limit,
                options.eventTypes,
                options.messageStatus,
                options.before
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
