package com.svix.kotlin

import com.svix.kotlin.internal.models.MessageStatus

class MessageAttemptListOptions() : ListOptions() {
	var messageStatus: MessageStatus? = null
}
