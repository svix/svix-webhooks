package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ApplicationPatch
import com.svix.kotlin.models.ListResponseApplicationOut

class Application internal constructor(token: String, options: SvixOptions) {
    private val api = ApplicationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(options: ApplicationListOptions = ApplicationListOptions()): ListResponseApplicationOut {
        try {
            return api.v1ApplicationList(options.limit, options.iterator, options.order)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(
        applicationIn: ApplicationIn,
        options: PostOptions = PostOptions(),
    ): ApplicationOut {
        try {
            return api.v1ApplicationCreate(applicationIn, null, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getOrCreate(
        applicationIn: ApplicationIn,
        options: PostOptions = PostOptions(),
    ): ApplicationOut {
        try {
            return api.v1ApplicationCreate(applicationIn, true, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String): ApplicationOut {
        try {
            return api.v1ApplicationGet(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(
        appId: String,
        applicationIn: ApplicationIn,
    ): ApplicationOut {
        try {
            return api.v1ApplicationUpdate(appId, applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun patch(
        appId: String,
        applicationPatch: ApplicationPatch,
    ): ApplicationOut {
        try {
            return api.v1ApplicationPatch(appId, applicationPatch)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String) {
        try {
            api.v1ApplicationDelete(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
