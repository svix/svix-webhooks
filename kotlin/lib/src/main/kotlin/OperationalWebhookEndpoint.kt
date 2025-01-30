// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.WebhookEndpointApi as OperationalWebhookEndpointApi
import com.svix.kotlin.models.ListResponseOperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointHeadersIn
import com.svix.kotlin.models.OperationalWebhookEndpointHeadersOut
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
    private val api = OperationalWebhookEndpointApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** List operational webhook endpoints. */
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

    /** Create an operational webhook endpoint. */
    suspend fun create(
        operationalWebhookEndpointIn: OperationalWebhookEndpointIn,
        options: PostOptions = PostOptions(),
    ): OperationalWebhookEndpointOut {
        try {
            return api.v1OperationalWebhookEndpointCreate(
                operationalWebhookEndpointIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get an operational webhook endpoint. */
    suspend fun get(endpointId: String): OperationalWebhookEndpointOut {
        try {
            return api.v1OperationalWebhookEndpointGet(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Update an operational webhook endpoint. */
    suspend fun update(
        endpointId: String,
        operationalWebhookEndpointUpdate: OperationalWebhookEndpointUpdate,
    ): OperationalWebhookEndpointOut {
        try {
            return api.v1OperationalWebhookEndpointUpdate(
                endpointId,
                operationalWebhookEndpointUpdate,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Delete an operational webhook endpoint. */
    suspend fun delete(endpointId: String) {
        try {
            api.v1OperationalWebhookEndpointDelete(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get the additional headers to be sent with the operational webhook. */
    suspend fun getHeaders(endpointId: String): OperationalWebhookEndpointHeadersOut {
        try {
            return api.v1OperationalWebhookEndpointGetHeaders(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Set the additional headers to be sent with the operational webhook. */
    suspend fun updateHeaders(
        endpointId: String,
        operationalWebhookEndpointHeadersIn: OperationalWebhookEndpointHeadersIn,
    ) {
        try {
            api.v1OperationalWebhookEndpointUpdateHeaders(
                endpointId,
                operationalWebhookEndpointHeadersIn,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Get an operational webhook endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook. For more information please refer to
     * [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    suspend fun getSecret(endpointId: String): OperationalWebhookEndpointSecretOut {
        try {
            return api.v1OperationalWebhookEndpointGetSecret(endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Rotates an operational webhook endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    suspend fun rotateSecret(
        endpointId: String,
        operationalWebhookEndpointSecretIn: OperationalWebhookEndpointSecretIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1OperationalWebhookEndpointRotateSecret(
                endpointId,
                operationalWebhookEndpointSecretIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
