package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EndpointApi
import com.svix.kotlin.models.EndpointHeadersIn
import com.svix.kotlin.models.EndpointHeadersOut
import com.svix.kotlin.models.EndpointHeadersPatchIn
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.EndpointPatch
import com.svix.kotlin.models.EndpointSecretOut
import com.svix.kotlin.models.EndpointSecretRotateIn
import com.svix.kotlin.models.EndpointStats
import com.svix.kotlin.models.EndpointTransformationIn
import com.svix.kotlin.models.EndpointTransformationOut
import com.svix.kotlin.models.EndpointUpdate
import com.svix.kotlin.models.EventExampleIn
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
        options: EndpointListOptions = EndpointListOptions(),
    ): ListResponseEndpointOut {
        try {
            return api.v1EndpointList(
                appId,
                options.limit,
                options.iterator,
                options.order,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(
        appId: String,
        endpointIn: EndpointIn,
        options: PostOptions = PostOptions(),
    ): EndpointOut {
        try {
            return api.v1EndpointCreate(
                appId,
                endpointIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(
        appId: String,
        endpointId: String,
    ): EndpointOut {
        try {
            return api.v1EndpointGet(endpointId, appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(
        appId: String,
        endpointId: String,
        endpointUpdate: EndpointUpdate,
    ): EndpointOut {
        try {
            return api.v1EndpointUpdate(
                appId,
                endpointId,
                endpointUpdate,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun patch(
        appId: String,
        endpointId: String,
        endpointPatch: EndpointPatch,
    ): EndpointOut {
        try {
            return api.v1EndpointPatch(
                appId,
                endpointId,
                endpointPatch,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(
        appId: String,
        endpointId: String,
    ) {
        try {
            api.v1EndpointDelete(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getSecret(
        appId: String,
        endpointId: String,
    ): EndpointSecretOut {
        try {
            return api.v1EndpointGetSecret(
                appId,
                endpointId,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun rotateSecret(
        appId: String,
        endpointId: String,
        endpointSecretRotateIn: EndpointSecretRotateIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1EndpointRotateSecret(
                appId,
                endpointId,
                endpointSecretRotateIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun recover(
        appId: String,
        endpointId: String,
        recoverIn: RecoverIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1EndpointRecover(
                appId,
                endpointId,
                recoverIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getHeaders(
        appId: String,
        endpointId: String,
    ): EndpointHeadersOut {
        try {
            return api.v1EndpointGetHeaders(
                appId,
                endpointId,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun updateHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersIn: EndpointHeadersIn,
    ) {
        try {
            api.v1EndpointUpdateHeaders(
                appId,
                endpointId,
                endpointHeadersIn,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun patchHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersIn: EndpointHeadersPatchIn,
    ) {
        try {
            api.v1EndpointPatchHeaders(
                appId,
                endpointId,
                endpointHeadersIn,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getStats(
        appId: String,
        endpointId: String,
        options: EndpointStatsOptions = EndpointStatsOptions(),
    ): EndpointStats {
        try {
            return api.v1EndpointGetStats(
                appId,
                endpointId,
                options.since,
                options.until,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun replayMissing(
        appId: String,
        endpointId: String,
        replayIn: ReplayIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1EndpointReplay(
                appId,
                endpointId,
                replayIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun transformationGet(
        appId: String,
        endpointId: String,
    ): EndpointTransformationOut {
        try {
            return api.v1EndpointTransformationGet(
                appId,
                endpointId,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun transformationPartialUpdate(
        appId: String,
        endpointId: String,
        endpointTransformationIn: EndpointTransformationIn,
    ) {
        try {
            api.v1EndpointTransformationPartialUpdate(
                appId,
                endpointId,
                endpointTransformationIn,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun sendExample(
        appId: String,
        endpointId: String,
        eventExampleIn: EventExampleIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1EndpointSendExample(
                appId,
                endpointId,
                eventExampleIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
