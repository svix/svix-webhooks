package com.svix;

public class EventTypeListOptions extends ListOptions {
    private boolean withContent;

	public EventTypeListOptions() {
        super();
	}

	public EventTypeListOptions withContent(final boolean withContent) {
		this.withContent = withContent;
		return this;
	}

	public void setWithContent(final boolean withContent) {
		this.withContent = withContent;
	}

	public boolean getWithContent() {
		return withContent;
	}
}
