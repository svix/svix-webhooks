package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.models.DashboardAccessOut

class Authentication internal constructor(debugUrl: String) {
    val api = AuthenticationApi(debugUrl)

    suspend fun dashboardAccess(appId: String): DashboardAccessOut {
        try {
            return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun logout() {
        try {
            api.logoutApiV1AuthLogoutPost()
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }
}
