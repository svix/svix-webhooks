package com.svix;

public class FetchOptions {
    private static final int DEFAULT_LIMIT = 50;

    private String iterator;
    private Integer limit = DEFAULT_LIMIT;

	public FetchOptions() {
	}

	public FetchOptions iterator(final String iterator) {
		this.iterator = iterator;
		return this;
	}

	public FetchOptions limit(final Integer limit) {
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
