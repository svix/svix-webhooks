package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.WebhookEndpointApi as OperationalWebhookEndpointApi
import com.svix.kotlin.models.ListResponseOperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointIn
import com.svix.kotlin.models.OperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointSecretIn
import com.svix.kotlin.models.OperationalWebhookEndpointSecretOut
import com.svix.kotlin.models.OperationalWebhookEndpointUpdate

class OperationalWebhookEndpoint internal constructor(token: String, options: SvixOptions) {
    val api = OperationalWebhookEndpointApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions()
    ): ListResponseOperationalWebhookEndpointOut {
        try {
            return api.listOperationalWebhookEndpoints(
                options.limit,
                options.iterator,
                options.order,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(
        endpointIn: OperationalWebhookEndpointIn,
        options: PostOptions = PostOptions(),
    ): OperationalWebhookEndpointOut {
        try {
            return api.createOperationalWebhookEndpoint(endpointIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(endpointId: String): OperationalWebhookEndpointOut {
        try {
            return api.getOperationalWebhookEndpoint(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(
        endpointId: String,
        endpointUpdate: OperationalWebhookEndpointUpdate,
    ): OperationalWebhookEndpointOut {
        try {
            return api.updateOperationalWebhookEndpoint(endpointId, endpointUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(endpointId: String) {
        try {
            api.deleteOperationalWebhookEndpoint(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getSecret(endpointId: String): OperationalWebhookEndpointSecretOut {
        try {
            return api.getOperationalWebhookEndpointSecret(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun rotateSecret(
        endpointId: String,
        endpointSecretRotateIn: OperationalWebhookEndpointSecretIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.rotateOperationalWebhookEndpointSecret(
                endpointId,
                endpointSecretRotateIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
