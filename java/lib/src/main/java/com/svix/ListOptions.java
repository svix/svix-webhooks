package com.svix;

public class ListOptions {
    private static final int DEFAULT_LIMIT = 50;

    private String iterator;
    private Integer limit = DEFAULT_LIMIT;

	public ListOptions() {
	}

	public <T extends ListOptions> T iterator(final String iterator) {
		this.iterator = iterator;
		return (T) this;
	}

	public <T extends ListOptions> T  limit(final Integer limit) {
		this.limit = limit;
		return (T) this;
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
