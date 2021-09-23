package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.models.DashboardAccessOut

class Authentication internal constructor(token: String, options: SvixOptions) {
    val api = AuthenticationApi(options.debugUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun dashboardAccess(appId: String): DashboardAccessOut {
        try {
            return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun logout() {
        try {
            api.logoutApiV1AuthLogoutPost()
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
