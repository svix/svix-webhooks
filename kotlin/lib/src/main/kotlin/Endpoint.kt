package com.svix.kotlin

import com.svix.kotlin.internal.apis.EndpointApi
import com.svix.kotlin.internal.models.EndpointIn
import com.svix.kotlin.internal.models.EndpointOut
import com.svix.kotlin.internal.models.ListResponseEndpointOut
import com.svix.kotlin.internal.infrastructure.ServerException

class Endpoint() {
	val api = EndpointApi()

	suspend fun list(appId: String, options: EndpointListOptions): ListResponseEndpointOut {
		try {
			return api.listEndpointsApiV1AppAppIdEndpointGet(appId, options.iterator, options.limit)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun create(appId: String, applicationIn: EndpointIn): EndpointOut {
		try {
			return api.createEndpointApiV1AppAppIdEndpointPost(appId, applicationIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun get(appId: String, endpointIn: String): EndpointOut {
		try {
			return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(appId, endpointIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun update(appId: String, endpointId: String, endpointIn: EndpointIn): EndpointOut {
		try {
			return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(appId, endpointId, endpointIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun delete(appId: String, endpointId: String) {
		try {
			api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(appId, endpointId)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun getSecret(appId: String, endpointId: String) {
		try {
			api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(appId, endpointId)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}
}
