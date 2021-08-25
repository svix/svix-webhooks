package com.svix;

public class ListOptions {
    private static final int DEFAULT_LIMIT = 50;

    private String iterator;
    private Integer limit = DEFAULT_LIMIT;

	public ListOptions() {
	}

	public ListOptions iterator(final String iterator) {
		this.iterator = iterator;
		return this;
	}

	public ListOptions limit(final Integer limit) {
		this.limit = limit;
		return this;
	}

	public void setIterator(final String iterator) {
		this.iterator = iterator;
	}

	public void setLimit(final Integer limit) {
		this.limit = limit;
	}

	public String getIterator() {
		return iterator;
	}

	public Integer getLimit() {
		return limit;
	}
}
