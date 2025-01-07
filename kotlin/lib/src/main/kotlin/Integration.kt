// this file is @generated (with minor manual changes)
package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.IntegrationApi
import com.svix.kotlin.models.IntegrationIn
import com.svix.kotlin.models.IntegrationKeyOut
import com.svix.kotlin.models.IntegrationOut
import com.svix.kotlin.models.IntegrationUpdate
import com.svix.kotlin.models.ListResponseIntegrationOut
import com.svix.kotlin.models.Ordering

class IntegrationListOptions {
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

class Integration internal constructor(token: String, options: SvixOptions) {
    private val api = IntegrationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** List the application's integrations. */
    suspend fun list(
        appId: String,
        options: IntegrationListOptions = IntegrationListOptions(),
    ): ListResponseIntegrationOut {
        try {
            return api.v1IntegrationList(appId, options.limit, options.iterator, options.order)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Create an integration. */
    suspend fun create(
        appId: String,
        integrationIn: IntegrationIn,
        options: PostOptions = PostOptions(),
    ): IntegrationOut {
        try {
            return api.v1IntegrationCreate(appId, integrationIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get an integration. */
    suspend fun get(appId: String, integId: String): IntegrationOut {
        try {
            return api.v1IntegrationGet(appId, integId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Update an integration. */
    suspend fun update(
        appId: String,
        integId: String,
        integrationUpdate: IntegrationUpdate,
    ): IntegrationOut {
        try {
            return api.v1IntegrationUpdate(appId, integId, integrationUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Delete an integration. */
    suspend fun delete(appId: String, integId: String) {
        try {
            api.v1IntegrationDelete(appId, integId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getKey(appId: String, integId: String): IntegrationKeyOut {
        try {
            return api.v1IntegrationGetKey(appId, integId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Rotate the integration's key. The previous key will be immediately revoked. */
    suspend fun rotateKey(
        appId: String,
        integId: String,
        options: PostOptions = PostOptions(),
    ): IntegrationKeyOut {
        try {
            return api.v1IntegrationRotateKey(appId, integId, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
