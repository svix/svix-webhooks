package com.svix.kotlin

open class ListOptions(var iterator: String = "", var limit: Int = 50) {
	  fun iterator(iterator : String) = apply { this.iterator = iterator}
	
	  fun limit(limit : Int) = apply { this.limit = limit}
}
