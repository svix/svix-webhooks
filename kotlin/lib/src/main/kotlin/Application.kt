package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ListResponseApplicationOut

class Application() {
    val api = ApplicationApi()

    suspend fun list(options: ApplicationListOptions): ListResponseApplicationOut {
        try {
            return api.listApplicationsApiV1AppGet(options.iterator, options.limit)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun create(applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.createApplicationApiV1AppPost(applicationIn)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun get(appId: String): ApplicationOut {
        try {
            return api.getApplicationApiV1AppAppIdGet(appId)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun delete(appId: String) {
        try {
            api.deleteApplicationApiV1AppAppIdDelete(appId)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }
}
