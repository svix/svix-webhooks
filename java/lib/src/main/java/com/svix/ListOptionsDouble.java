package com.svix;

public class ListOptionsDouble extends ListOptions {
    private String prevIterator;

	public ListOptionsDouble() {
	}

	public <T extends ListOptionsDouble> T prevIterator(final String iterator) {
		this.prevIterator = prevIterator;
		return (T) this;
	}

	public void setPrevIterator(final String iterator) {
		this.prevIterator = iterator;
	}

	public String getPrevIterator() {
		return prevIterator;
	}
}
