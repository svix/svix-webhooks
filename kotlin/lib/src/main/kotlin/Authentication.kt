package com.svix.kotlin

import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.internal.models.DashboardAccessOut

class Authentication() {
    val api = AuthenticationApi()

    suspend fun dashboardAccess(appId: String): DashboardAccessOut {
        try {
            return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId)
        } catch (ex: Exception) {
            // TODO: Wrap expeption with new expection
            throw ServerException(ex.message)
        }
    }

    suspend fun logout() {
        try {
            api.logoutApiV1AuthLogoutPost()
        } catch (ex: Exception) {
            // TODO: Wrap expeption with new expection
            throw ServerException(ex.message)
        }
    }
}
