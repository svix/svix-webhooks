package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EndpointApi
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.EndpointUpdate
import com.svix.kotlin.models.ListResponseEndpointOut

class Endpoint internal constructor(debugUrl: String) {
    val api = EndpointApi(debugUrl)

    suspend fun list(appId: String, options: EndpointListOptions): ListResponseEndpointOut {
        try {
            return api.listEndpointsApiV1AppAppIdEndpointGet(appId, options.iterator, options.limit)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun create(appId: String, applicationIn: EndpointIn): EndpointOut {
        try {
            return api.createEndpointApiV1AppAppIdEndpointPost(appId, applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun get(appId: String, endpointIn: String): EndpointOut {
        try {
            return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(appId, endpointIn)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun update(appId: String, endpointId: String, endpointUpdate: EndpointUpdate): EndpointOut {
        try {
            return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(appId, endpointId, endpointUpdate)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun delete(appId: String, endpointId: String) {
        try {
            api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun getSecret(appId: String, endpointId: String) {
        try {
            api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }
}
