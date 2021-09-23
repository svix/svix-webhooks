package com.svix.kotlin

class EventTypeListOptions() : ListOptions() {
    var withContent: Boolean? = null

    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
