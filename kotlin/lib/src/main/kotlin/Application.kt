package com.svix.kotlin

import com.svix.kotlin.internal.apis.ApplicationApi
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.internal.models.ApplicationIn
import com.svix.kotlin.internal.models.ApplicationOut
import com.svix.kotlin.internal.models.ListResponseApplicationOut

class Application() {
	val api = ApplicationApi()

	suspend fun list(options: ApplicationListOptions): ListResponseApplicationOut {
		try {
			return api.listApplicationsApiV1AppGet(options.iterator, options.limit)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun create(applicationIn: ApplicationIn): ApplicationOut {
		try {
			return api.createApplicationApiV1AppPost(applicationIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun get(appId: String): ApplicationOut {
		try {
			return api.getApplicationApiV1AppAppIdGet(appId)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
		try {
			return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun delete(appId: String) {
		try {
			api.deleteApplicationApiV1AppAppIdDelete(appId)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}
}
