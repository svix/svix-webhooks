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
    }

    suspend fun list(appId: String, options: IntegrationListOptions = IntegrationListOptions()): ListResponseIntegrationOut {
        try {
            return api.listIntegrationsApiV1AppAppIdIntegrationGet(appId, options.iterator, options.limit)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(appId: String, integIn: IntegrationIn): IntegrationOut {
        try {
            return api.createIntegrationApiV1AppAppIdIntegrationPost(appId, integIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String, integId: String): IntegrationOut {
        try {
            return api.getIntegrationApiV1AppAppIdIntegrationIntegIdGet(integId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(appId: String, integId: String, integUpdate: IntegrationUpdate): IntegrationOut {
        try {
            return api.updateIntegrationApiV1AppAppIdIntegrationIntegIdPut(integId, appId, integUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String, integId: String) {
        try {
            api.deleteIntegrationApiV1AppAppIdIntegrationIntegIdDelete(integId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getKey(appId: String, integId: String): IntegrationKeyOut {
        try {
            return api.getIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(integId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun rotateKey(appId: String, integId: String): IntegrationKeyOut {
        try {
            return api.rotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(integId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
