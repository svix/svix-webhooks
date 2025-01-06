// this file is @generated (with minor manual changes)
package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ApplicationPatch
import com.svix.kotlin.models.ListResponseApplicationOut
import com.svix.kotlin.models.Ordering

class ApplicationListOptions {
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

class Application internal constructor(token: String, options: SvixOptions) {
    private val api = ApplicationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** List of all the organization's applications. */
    suspend fun list(
        options: ApplicationListOptions = ApplicationListOptions()
    ): ListResponseApplicationOut {
        try {
            return api.v1ApplicationList(options.limit, options.iterator, options.order)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Create a new application. */
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

    /** Get an application. */
    suspend fun get(appId: String): ApplicationOut {
        try {
            return api.v1ApplicationGet(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Update an application. */
    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.v1ApplicationUpdate(appId, applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Delete an application. */
    suspend fun delete(appId: String) {
        try {
            api.v1ApplicationDelete(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Partially update an application. */
    suspend fun patch(appId: String, applicationPatch: ApplicationPatch): ApplicationOut {
        try {
            return api.v1ApplicationPatch(appId, applicationPatch)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
