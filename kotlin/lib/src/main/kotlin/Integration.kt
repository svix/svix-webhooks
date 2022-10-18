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

    suspend fun list(appId: String, options: IntegrationListOptions = IntegrationListOptions()): ListResponseIntegrationOut {
        try {
            return api.listIntegrationsApiV1AppAppIdIntegrationGet(appId, options.iterator, options.limit, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(appId: String, integIn: IntegrationIn, options: PostOptions = PostOptions()): IntegrationOut {
        try {
            return api.createIntegrationApiV1AppAppIdIntegrationPost(appId, integIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String, integId: String): IntegrationOut {
        try {
            return api.getIntegrationApiV1AppAppIdIntegrationIntegIdGet(integId, appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(appId: String, integId: String, integUpdate: IntegrationUpdate): IntegrationOut {
        try {
            return api.updateIntegrationApiV1AppAppIdIntegrationIntegIdPut(integId, appId, integUpdate, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String, integId: String) {
        try {
            api.deleteIntegrationApiV1AppAppIdIntegrationIntegIdDelete(integId, appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getKey(appId: String, integId: String): IntegrationKeyOut {
        try {
            return api.getIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(integId, appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun rotateKey(appId: String, integId: String, options: PostOptions = PostOptions()): IntegrationKeyOut {
        try {
            return api.rotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(
                integId,
                appId,
                options.idempotencyKey
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
