package com.svix.kotlin

import com.svix.kotlin.models.MessageStatus
import com.svix.kotlin.models.StatusCodeClass
import java.time.OffsetDateTime

class MessageAttemptListOptions(
    var messageStatus: MessageStatus? = null,
    var before: OffsetDateTime? = null,
    var after: OffsetDateTime? = null,
    var eventTypes: List<String>? = null,
    var statusCodeClass: StatusCodeClass? = null,
    var channel: String? = null,
    var tag: String? = null,
    var endpointId: String? = null,
    var withContent: Boolean? = null,
    var withMsg: Boolean? = null,
) : ListOptions() {
    fun messageStatus(messageStatus: MessageStatus) = apply { this.messageStatus = messageStatus }

    fun before(before: OffsetDateTime) = apply { this.before = before }

    fun after(after: OffsetDateTime) = apply { this.after = after }

    fun statusCodeClass(statusCodeClass: StatusCodeClass) = apply { this.statusCodeClass = statusCodeClass }

    fun eventTypes(eventTypes: List<String>) = apply { this.eventTypes = eventTypes }

    fun channel(channel: String) = apply { this.channel = channel }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }

    fun endpointId(endpointId: String) = apply { this.endpointId = endpointId }

    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    fun withMsg(withMsg: Boolean) = apply { this.withMsg = withMsg }

    fun tag(tag: String) = apply { this.tag = tag }
}
