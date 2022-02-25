package com.svix.kotlin

import com.svix.kotlin.models.MessageStatus
import java.time.OffsetDateTime

class MessageAttemptListOptions(var messageStatus: MessageStatus? = null, var before: OffsetDateTime? = null, var after: OffsetDateTime? = null, var eventTypes: List<String>? = null) : ListOptionsDouble() {
    fun messageStatus(messageStatus: MessageStatus) = apply { this.messageStatus = messageStatus }

    fun before(before: OffsetDateTime) = apply { this.before = before }
    fun after(after: OffsetDateTime) = apply { this.after = after }

    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }
    override fun prevIterator(iterator: String) = apply { super.prevIterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
