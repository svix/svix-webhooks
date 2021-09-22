package com.svix.kotlin

import com.svix.kotlin.models.MessageStatus
import java.time.OffsetDateTime

class MessageAttemptListOptions(var messageStatus: MessageStatus? = null, var before: OffsetDateTime? = null, var eventTypes: List<String>? = null) : ListOptions() {
    fun messageStatus(messageStatus: MessageStatus) = apply { this.messageStatus = messageStatus }

    fun before(before: OffsetDateTime) = apply { this.before = before }

    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
