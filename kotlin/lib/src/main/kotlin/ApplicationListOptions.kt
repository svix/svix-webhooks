package com.svix.kotlin

class ApplicationListOptions : ListOptions() {
    override fun iterator(iterator: String): ApplicationListOptions = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
