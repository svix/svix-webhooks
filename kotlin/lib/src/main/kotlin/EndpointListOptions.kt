package com.svix.kotlin

import com.svix.kotlin.models.Ordering

class EndpointListOptions : ListOptions() {
    var order: Ordering? = null

    fun order(order: Ordering) = apply { this.order = order }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
