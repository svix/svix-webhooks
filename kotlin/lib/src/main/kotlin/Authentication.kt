package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.models.DashboardAccessOut

class Authentication(debugUrl: String = SvixOptions.DEFAULT_URL) {
    val api = AuthenticationApi(debugUrl)

    suspend fun dashboardAccess(appId: String): DashboardAccessOut {
        try {
            return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId)
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }

    suspend fun logout() {
        try {
            api.logoutApiV1AuthLogoutPost()
        } catch (ex: Exception) {
            throw ApiException(ex)
        }
    }
}
