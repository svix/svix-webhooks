package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageAttemptApi
import com.svix.kotlin.models.MessageAttemptOut
import com.svix.kotlin.models.ListResponseEndpointMessageOut
import com.svix.kotlin.models.ListResponseMessageAttemptEndpointOut
import com.svix.kotlin.models.ListResponseMessageAttemptOut
import com.svix.kotlin.models.ListResponseMessageEndpointOut

class MessageAttempt() {
    val api = MessageAttemptApi()

    suspend fun list(appId: String, msgId: String, options: MessageAttemptListOptions): ListResponseMessageAttemptOut {
        try {
            return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(appId, msgId, options.iterator, options.limit, options.messageStatus)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun get(msgId: String, appId: String, attemptId: String): MessageAttemptOut {
        try {
            return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun resend(msgId: String, appId: String, endpointId: String) {
        try {
            api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun listAttemptedMessages(appId: String, endpointId: String, options: MessageAttemptListOptions): ListResponseEndpointMessageOut {
        try {
            return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(appId, endpointId, options.iterator, options.limit, options.messageStatus)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun listAttemptedDestinations(msgId: String, appId: String, options: MessageAttemptListOptions): ListResponseMessageEndpointOut {
        try {
            return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, options.iterator, options.limit)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun listAttemptsForEndpoint(msgId: String, appId: String, endpointId: String, options: MessageAttemptListOptions): ListResponseMessageAttemptEndpointOut {
        try {
            return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId, options.iterator, options.limit, options.messageStatus)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }
}
