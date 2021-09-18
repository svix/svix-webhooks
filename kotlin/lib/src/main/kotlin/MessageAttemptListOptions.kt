package com.svix.kotlin

import com.svix.kotlin.models.MessageStatus

class MessageAttemptListOptions(var messageStatus: MessageStatus? = null) : ListOptions()
