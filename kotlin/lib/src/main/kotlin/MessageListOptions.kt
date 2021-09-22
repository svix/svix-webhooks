package com.svix.kotlin

import java.time.OffsetDateTime

abstract class MessageListOptions : ListOptions() {
    abstract var eventTypes: List<String>?
    abstract var before: OffsetDateTime?
}
