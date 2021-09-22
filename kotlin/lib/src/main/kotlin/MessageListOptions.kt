package com.svix.kotlin

import java.time.OffsetDateTime

class MessageListOptions(var eventTypes: List<String>? = null, var before: OffsetDateTime) : ListOptions() {
    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    fun before(before: OffsetDateTime) = apply { this.before = before }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
