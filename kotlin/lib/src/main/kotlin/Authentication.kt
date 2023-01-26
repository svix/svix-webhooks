package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.models.AppPortalAccessIn
import com.svix.kotlin.models.AppPortalAccessOut
import com.svix.kotlin.models.DashboardAccessOut

class Authentication internal constructor(token: String, options: SvixOptions) {
    val api = AuthenticationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun appPortalAccess(
        appId: String,
        appPortalAccessIn: AppPortalAccessIn,
        options: PostOptions = PostOptions()
    ): AppPortalAccessOut {
        try {
            return api.getAppPortalAccessApiV1AuthAppPortalAccessAppIdPost(appId, appPortalAccessIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun dashboardAccess(appId: String, options: PostOptions = PostOptions()): DashboardAccessOut {
        try {
            return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun logout(options: PostOptions = PostOptions()) {
        try {
            api.logoutApiV1AuthLogoutPost(options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
