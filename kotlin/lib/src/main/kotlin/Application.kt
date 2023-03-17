package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
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
            return api.listApplicationsApiV1AppGet(options.iterator, options.limit, options.order, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(applicationIn: ApplicationIn, options: PostOptions = PostOptions()): ApplicationOut {
        try {
            return api.createApplicationApiV1AppPost(applicationIn, null, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun getOrCreate(applicationIn: ApplicationIn, options: PostOptions = PostOptions()): ApplicationOut {
        try {
            return api.createApplicationApiV1AppPost(applicationIn, true, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String): ApplicationOut {
        try {
            return api.getApplicationApiV1AppAppIdGet(appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String) {
        try {
            api.deleteApplicationApiV1AppAppIdDelete(appId, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
