package com.svix.kotlin

import com.svix.kotlin.models.MessageStatus

class MessageAttemptListOptions() : ListOptions() {
	var messageStatus: MessageStatus? = null
}
