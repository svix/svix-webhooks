package com.svix.kotlin

import java.time.OffsetDateTime

class MessageListOptions : ListOptionsDouble() {
    var eventTypes: List<String>? = null
    var before: OffsetDateTime? = null
    var after: OffsetDateTime? = null
    var channel: String? = null

    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    fun before(before: OffsetDateTime) = apply { this.before = before }
    fun after(after: OffsetDateTime) = apply { this.after = after }

    fun channel(channel: String) = apply { this.channel = channel }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }
    override fun prevIterator(iterator: String) = apply { super.prevIterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
