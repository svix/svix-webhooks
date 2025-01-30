// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.BackgroundTasksApi
import com.svix.kotlin.models.BackgroundTaskOut
import com.svix.kotlin.models.BackgroundTaskStatus
import com.svix.kotlin.models.BackgroundTaskType
import com.svix.kotlin.models.ListResponseBackgroundTaskOut
import com.svix.kotlin.models.Ordering

class BackgroundTaskListOptions {
    var status: BackgroundTaskStatus? = null
    var task: BackgroundTaskType? = null
    var limit: Int? = null
    var iterator: String? = null
    var order: Ordering? = null

    /** Filter the response based on the status. */
    fun status(status: BackgroundTaskStatus) = apply { this.status = status }

    /** Filter the response based on the type. */
    fun task(task: BackgroundTaskType) = apply { this.task = task }

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }

    /** The sorting order of the returned items */
    fun order(order: Ordering) = apply { this.order = order }
}

class BackgroundTask internal constructor(token: String, options: SvixOptions) {
    private val api = BackgroundTasksApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** List background tasks executed in the past 90 days. */
    suspend fun list(
        options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ): ListResponseBackgroundTaskOut {
        try {
            return api.v1BackgroundTaskList(
                options.status,
                options.task,
                options.limit,
                options.iterator,
                options.order,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get a background task by ID. */
    suspend fun get(taskId: String): BackgroundTaskOut {
        try {
            return api.v1BackgroundTaskGet(taskId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
