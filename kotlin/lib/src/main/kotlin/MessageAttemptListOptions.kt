package com.svix.kotlin

import com.svix.kotlin.models.MessageStatus
import java.time.OffsetDateTime

abstract class MessageAttemptListOptions : ListOptions() {
    abstract var messageStatus: MessageStatus?
    abstract var before: OffsetDateTime?
    abstract var eventTypes: List<String>?
}
