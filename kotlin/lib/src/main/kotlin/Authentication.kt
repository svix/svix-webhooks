// this file is @generated (with minor manual changes)
package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.AuthenticationApi
import com.svix.kotlin.models.AppPortalAccessIn
import com.svix.kotlin.models.AppPortalAccessOut
import com.svix.kotlin.models.ApplicationTokenExpireIn
import com.svix.kotlin.models.DashboardAccessOut

class Authentication internal constructor(token: String, options: SvixOptions) {
    private val api = AuthenticationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /**
     * Use this function to get magic links (and authentication codes) for connecting your users to
     * the Consumer Application Portal.
     */
    suspend fun appPortalAccess(
        appId: String,
        appPortalAccessIn: AppPortalAccessIn,
        options: PostOptions = PostOptions(),
    ): AppPortalAccessOut {
        try {
            return api.v1AuthenticationAppPortalAccess(
                appId,
                appPortalAccessIn,
                options.idempotencyKey,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun dashboardAccess(
        appId: String,
        options: PostOptions = PostOptions(),
    ): DashboardAccessOut {
        try {
            return api.v1AuthenticationDashboardAccess(appId, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Expire all of the tokens associated with a specific application. */
    suspend fun expireAll(
        appId: String,
        applicationTokenExpireIn: ApplicationTokenExpireIn,
        options: PostOptions = PostOptions(),
    ) {
        try {
            api.v1AuthenticationExpireAll(appId, applicationTokenExpireIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Logout an app token.
     *
     * Trying to log out other tokens will fail.
     */
    suspend fun logout(options: PostOptions = PostOptions()) {
        try {
            api.v1AuthenticationLogout(options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
