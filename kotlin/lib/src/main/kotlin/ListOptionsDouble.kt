package com.svix.kotlin

open class ListOptionsDouble : ListOptions() {
    var prevIterator: String? = null

    open fun prevIterator(iterator: String) = apply { this.prevIterator = iterator }
}
