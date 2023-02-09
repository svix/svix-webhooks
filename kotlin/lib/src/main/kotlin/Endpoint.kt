package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EndpointApi
import com.svix.kotlin.models.EndpointHeadersIn
import com.svix.kotlin.models.EndpointHeadersOut
import com.svix.kotlin.models.EndpointHeadersPatchIn
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.EndpointSecretOut
import com.svix.kotlin.models.EndpointSecretRotateIn
import com.svix.kotlin.models.EndpointStats
import com.svix.kotlin.models.EndpointTransformationIn
import com.svix.kotlin.models.EndpointTransformationOut
import com.svix.kotlin.models.EndpointUpdate
import com.svix.kotlin.models.ListResponseEndpointOut
import com.svix.kotlin.models.RecoverIn
import com.svix.kotlin.models.ReplayIn

class Endpoint internal constructor(token: String, options: SvixOptions) {
    val api = EndpointApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(
        appId: String,
        options: EndpointListOptions = EndpointListOptions()
    ): ListResponseEndpointOut {
        try {
            return api.listEndpointsApiV1AppAppIdEndpointGet(
                appId,
                options.iterator,
                options.limit,
                options.order,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(
        appId: String,
        endpointIn: EndpointIn,
        options: PostOptions = PostOptions()
    ): EndpointOut {
        try {
            return api.createEndpointApiV1AppAppIdEndpointPost(
                appId,
                endpointIn,
                options.idempotencyKey
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String, endpointId: String): EndpointOut {
        try {
            return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(
        appId: String,
        endpointId: String,
        endpointUpdate: EndpointUpdate
    ): EndpointOut {
        try {
            return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(
                endpointId,
                appId,
                endpointUpdate,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String, endpointId: String) {
        try {
            api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getSecret(appId: String, endpointId: String): EndpointSecretOut {
        try {
            return api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(
                endpointId,
                appId,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun rotateSecret(
        appId: String,
        endpointId: String,
        endpointSecretRotateIn: EndpointSecretRotateIn,
        options: PostOptions = PostOptions()
    ) {
        try {
            api.rotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePost(
                endpointId,
                appId,
                endpointSecretRotateIn,
                options.idempotencyKey
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun recover(
        appId: String,
        endpointId: String,
        recoverIn: RecoverIn,
        options: PostOptions = PostOptions()
    ) {
        try {
            api.recoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPost(
                appId,
                endpointId,
                recoverIn,
                options.idempotencyKey
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getHeaders(appId: String, endpointId: String): EndpointHeadersOut {
        try {
            return api.getEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGet(
                endpointId,
                appId,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun updateHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersIn: EndpointHeadersIn
    ) {
        try {
            api.updateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPut(
                appId,
                endpointId,
                endpointHeadersIn,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun patchHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersIn: EndpointHeadersPatchIn
    ) {
        try {
            api.patchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatch(
                appId,
                endpointId,
                endpointHeadersIn,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getStats(appId: String, endpointId: String): EndpointStats {
        try {
            return api.getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet(
                endpointId,
                appId,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun replayMissing(
        appId: String,
        endpointId: String,
        replayIn: ReplayIn,
        options: PostOptions = PostOptions()
    ) {
        try {
            api.replayMissingWebhooksApiV1AppAppIdEndpointEndpointIdReplayMissingPost(
                appId,
                endpointId,
                replayIn,
                options.idempotencyKey
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun transformationGet(appId: String, endpointId: String): EndpointTransformationOut {
        try {
            return api.getEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationGet(
                endpointId,
                appId,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun transformationPartialUpdate(appId: String, endpointId: String, endpointTransformationIn: EndpointTransformationIn) {
        try {
            api.setEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationPatch(
                endpointId,
                appId,
                endpointTransformationIn,
                null
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
