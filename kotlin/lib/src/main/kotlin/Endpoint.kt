package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EndpointApi
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.EndpointUpdate
import com.svix.kotlin.models.ListResponseEndpointOut

class Endpoint internal constructor(token: String, options: SvixOptions) {
    val api = EndpointApi(options.debugUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun list(appId: String, options: EndpointListOptions = EndpointListOptions()): ListResponseEndpointOut {
        try {
            return api.listEndpointsApiV1AppAppIdEndpointGet(appId, options.iterator, options.limit)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(appId: String, endpointIn: EndpointIn): EndpointOut {
        try {
            return api.createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String, endpointId: String): EndpointOut {
        try {
            return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(appId: String, endpointId: String, endpointUpdate: EndpointUpdate): EndpointOut {
        try {
            return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String, endpointId: String) {
        try {
            api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getSecret(appId: String, endpointId: String) {
        try {
            api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
