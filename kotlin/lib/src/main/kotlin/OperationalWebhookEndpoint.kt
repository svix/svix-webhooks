package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.WebhookEndpointApi as OperationalWebhookEndpointApi
import com.svix.kotlin.models.ListResponseOperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointIn
import com.svix.kotlin.models.OperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointSecretIn
import com.svix.kotlin.models.OperationalWebhookEndpointSecretOut
import com.svix.kotlin.models.OperationalWebhookEndpointUpdate
import com.svix.kotlin.models.Ordering

class OperationalWebhookEndpointListOptions {
    var limit: Int? = null
    var iterator: String? = null
    var order: Ordering? = null

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }

    /** The sorting order of the returned items */
    fun order(order: Ordering) = apply { this.order = order }
}

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
            return api.v1OperationalWebhookEndpointList(
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
            return api.v1OperationalWebhookEndpointCreate(endpointIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(endpointId: String): OperationalWebhookEndpointOut {
        try {
            return api.v1OperationalWebhookEndpointGet(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(
        endpointId: String,
        endpointUpdate: OperationalWebhookEndpointUpdate,
    ): OperationalWebhookEndpointOut {
        try {
            return api.v1OperationalWebhookEndpointUpdate(endpointId, endpointUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(endpointId: String) {
        try {
            api.v1OperationalWebhookEndpointDelete(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getSecret(endpointId: String): OperationalWebhookEndpointSecretOut {
        try {
            return api.v1OperationalWebhookEndpointGetSecret(endpointId)
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
            api.v1OperationalWebhookEndpointRotateSecret(
                endpointId,
                endpointSecretRotateIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
