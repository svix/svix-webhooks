package com.svix.kotlin

class EventTypeListOptions() : ListOptions() {
    var withContent: Boolean? = null
    var includeAchived: Boolean? = null

    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    fun includeAchived(includeAchived: Boolean) = apply { this.includeAchived = includeAchived }

    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
