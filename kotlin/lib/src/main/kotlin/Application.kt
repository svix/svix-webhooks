package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ListResponseApplicationOut

class ApplicationListOptions() : ListOptions() {
    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}

class Application internal constructor(debugUrl: String) {
    val api = ApplicationApi(debugUrl)

    suspend fun list(options: ApplicationListOptions): ListResponseApplicationOut {
        try {
            return api.listApplicationsApiV1AppGet(options.iterator, options.limit)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun create(applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.createApplicationApiV1AppPost(applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun get(appId: String): ApplicationOut {
        try {
            return api.getApplicationApiV1AppAppIdGet(appId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        try {
            return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun delete(appId: String) {
        try {
            api.deleteApplicationApiV1AppAppIdDelete(appId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }
}
