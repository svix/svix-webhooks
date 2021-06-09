package com.svix;

public class FetchOptions {
    private static final int DEFAULT_LIMIT = 50;

    private String iterator;
    private Integer limit = DEFAULT_LIMIT;

	public FetchOptions() {
	}

	public FetchOptions setIterator(final String iterator) {
		this.iterator = iterator;
		return this;
	}

	public FetchOptions setLimit(final Integer limit) {
		this.limit = limit;
		return this;
	}

	public String getIterator() {
		return iterator;
	}

	public Integer getLimit() {
		return limit;
	}
}
