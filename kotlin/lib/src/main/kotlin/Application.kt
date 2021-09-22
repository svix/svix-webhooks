package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ListResponseApplicationOut

class Application internal constructor(token: String, options: SvixOptions) {
    private val api = ApplicationApi(options.debugUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun list(options: ApplicationListOptions = ApplicationListOptions()): ListResponseApplicationOut {
        try {
            return api.listApplicationsApiV1AppGet(options.iterator, options.limit)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.createApplicationApiV1AppPost(applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(appId: String): ApplicationOut {
        try {
            return api.getApplicationApiV1AppAppIdGet(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(appId: String) {
        try {
            api.deleteApplicationApiV1AppAppIdDelete(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
