// this file is @generated
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
import com.svix.kotlin.models.MessageOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.models.RecoverIn
import com.svix.kotlin.models.RecoverOut
import com.svix.kotlin.models.ReplayIn
import com.svix.kotlin.models.ReplayOut
import java.time.OffsetDateTime

class EndpointListOptions {
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

class EndpointGetStatsOptions {
    var since: OffsetDateTime? = null
    var until: OffsetDateTime? = null

    /** Filter the range to data starting from this date. */
    fun since(since: OffsetDateTime) = apply { this.since = since }

    /** Filter the range to data ending by this date. */
    fun until(until: OffsetDateTime) = apply { this.until = until }
}

class Endpoint internal constructor(token: String, options: SvixOptions) {
    private val api = EndpointApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** List the application's endpoints. */
    suspend fun list(
        appId: String,
        options: EndpointListOptions = EndpointListOptions(),
    ): ListResponseEndpointOut {
        try {
            return api.v1EndpointList(appId, options.limit, options.iterator, options.order)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Create a new endpoint for the application.
     *
     * When `secret` is `null` the secret is automatically generated (recommended).
     */
    suspend fun create(
        appId: String,
        endpointIn: EndpointIn,
        options: PostOptions = PostOptions(),
    ): EndpointOut {
        try {
            return api.v1EndpointCreate(appId, endpointIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get an endpoint. */
    suspend fun get(endpointId: String, appId: String): EndpointOut {
        try {
            return api.v1EndpointGet(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Update an endpoint. */
    suspend fun update(
        appId: String,
        endpointId: String,
        endpointUpdate: EndpointUpdate,
    ): EndpointOut {
        try {
            return api.v1EndpointUpdate(appId, endpointId, endpointUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Delete an endpoint. */
    suspend fun delete(appId: String, endpointId: String) {
        try {
            api.v1EndpointDelete(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Partially update an endpoint. */
    suspend fun patch(
        appId: String,
        endpointId: String,
        endpointPatch: EndpointPatch,
    ): EndpointOut {
        try {
            return api.v1EndpointPatch(appId, endpointId, endpointPatch)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get the additional headers to be sent with the webhook. */
    suspend fun getHeaders(appId: String, endpointId: String): EndpointHeadersOut {
        try {
            return api.v1EndpointGetHeaders(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Set the additional headers to be sent with the webhook. */
    suspend fun updateHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersIn: EndpointHeadersIn,
    ) {
        try {
            api.v1EndpointUpdateHeaders(appId, endpointId, endpointHeadersIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Partially set the additional headers to be sent with the webhook. */
    suspend fun patchHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersPatchIn: EndpointHeadersPatchIn,
    ) {
        try {
            api.v1EndpointPatchHeaders(appId, endpointId, endpointHeadersPatchIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Resend all failed messages since a given time.
     *
     * Messages that were sent successfully, even if failed initially, are not resent.
     */
    suspend fun recover(
        appId: String,
        endpointId: String,
        recoverIn: RecoverIn,
        options: PostOptions = PostOptions(),
    ): RecoverOut {
        try {
            return api.v1EndpointRecover(appId, endpointId, recoverIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Replays messages to the endpoint.
     *
     * Only messages that were created after `since` will be sent. Messages that were previously
     * sent to the endpoint are not resent.
     */
    suspend fun replayMissing(
        appId: String,
        endpointId: String,
        replayIn: ReplayIn,
        options: PostOptions = PostOptions(),
    ): ReplayOut {
        try {
            return api.v1EndpointReplayMissing(appId, endpointId, replayIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Get the endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook. For more information please refer to
     * [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    suspend fun getSecret(appId: String, endpointId: String): EndpointSecretOut {
        try {
            return api.v1EndpointGetSecret(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Rotates the endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
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

    /** Send an example message for an event. */
    suspend fun sendExample(
        appId: String,
        endpointId: String,
        eventExampleIn: EventExampleIn,
        options: PostOptions = PostOptions(),
    ): MessageOut {
        try {
            return api.v1EndpointSendExample(
                appId,
                endpointId,
                eventExampleIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get basic statistics for the endpoint. */
    suspend fun getStats(
        appId: String,
        endpointId: String,
        options: EndpointGetStatsOptions = EndpointGetStatsOptions(),
    ): EndpointStats {
        try {
            return api.v1EndpointGetStats(appId, endpointId, options.since, options.until)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get the transformation code associated with this endpoint. */
    suspend fun transformationGet(appId: String, endpointId: String): EndpointTransformationOut {
        try {
            return api.v1EndpointTransformationGet(appId, endpointId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Set or unset the transformation code associated with this endpoint. */
    suspend fun transformationPartialUpdate(
        appId: String,
        endpointId: String,
        endpointTransformationIn: EndpointTransformationIn,
    ) {
        try {
            api.v1EndpointTransformationPartialUpdate(appId, endpointId, endpointTransformationIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
