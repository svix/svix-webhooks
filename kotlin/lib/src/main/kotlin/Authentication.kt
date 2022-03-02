package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.models.DashboardAccessOut

class Authentication internal constructor(token: String, options: SvixOptions) {
    val api = AuthenticationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
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
