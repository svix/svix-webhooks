package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.BackgroundTasksApi
import com.svix.kotlin.models.BackgroundTaskOut
import com.svix.kotlin.models.BackgroundTaskStatus
import com.svix.kotlin.models.BackgroundTaskType
import com.svix.kotlin.models.ListResponseBackgroundTaskOut
import com.svix.kotlin.models.Ordering

class BackgroundTaskListOptions : ListOptions() {
    var status: BackgroundTaskStatus? = null
    var task: BackgroundTaskType? = null
    var order: Ordering? = null

    fun order(order: Ordering) = apply { this.order = order }

    fun status(status: BackgroundTaskStatus) = apply { this.status = status }

    fun task(task: BackgroundTaskType) = apply { this.task = task }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}

class BackgroundTask internal constructor(token: String, options: SvixOptions) {
    private val api = BackgroundTasksApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(
        options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ): ListResponseBackgroundTaskOut {
        try {
            return api.listBackgroundTasks(
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

    suspend fun get(taskId: String): BackgroundTaskOut {
        try {
            return api.getBackgroundTask(taskId)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
