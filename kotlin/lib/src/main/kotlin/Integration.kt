package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.IntegrationApi
import com.svix.kotlin.models.IntegrationIn
import com.svix.kotlin.models.IntegrationKeyOut
import com.svix.kotlin.models.IntegrationOut
import com.svix.kotlin.models.IntegrationUpdate
import com.svix.kotlin.models.ListResponseIntegrationOut

class Integration internal constructor(token: String, options: SvixOptions) {
    val api = IntegrationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

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

    suspend fun create(
        appId: String,
        integIn: IntegrationIn,
        options: PostOptions = PostOptions(),
    ): IntegrationOut {
        try {
            return api.v1IntegrationCreate(appId, integIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(
        appId: String,
        integId: String,
    ): IntegrationOut {
        try {
            return api.v1IntegrationGet(appId, integId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(
        appId: String,
        integId: String,
        integUpdate: IntegrationUpdate,
    ): IntegrationOut {
        try {
            return api.v1IntegrationUpdate(appId, integId, integUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(
        appId: String,
        integId: String,
    ) {
        try {
            api.v1IntegrationDelete(appId, integId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getKey(
        appId: String,
        integId: String,
    ): IntegrationKeyOut {
        try {
            return api.v1IntegrationGetKey(appId, integId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun rotateKey(
        appId: String,
        integId: String,
        options: PostOptions = PostOptions(),
    ): IntegrationKeyOut {
        try {
            return api.v1IntegrationRotateKey(
                appId,
                integId,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
