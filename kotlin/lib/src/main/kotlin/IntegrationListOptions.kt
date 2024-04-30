package com.svix.kotlin

import com.svix.kotlin.models.Ordering

class IntegrationListOptions : ListOptions() {
    var order: Ordering? = null

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }

    fun order(order: Ordering) = apply { this.order = order }
}
