package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EnvironmentApi
import com.svix.kotlin.models.EnvironmentIn
import com.svix.kotlin.models.EnvironmentOut

class Environment internal constructor(token: String, options: SvixOptions) {
    private val api = EnvironmentApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun export(): EnvironmentOut {
        try {
            return api.exportEnvironmentConfigurationApiV1EnvironmentExportPost(Object())
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun import(environmentIn: EnvironmentIn) {
        try {
            api.importEnvironmentConfigurationApiV1EnvironmentImportPost(environmentIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
