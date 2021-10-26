package com.svix;

public class EventTypeListOptions extends ListOptions {
	private boolean withContent;
	private boolean includeArchived;

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

	public EventTypeListOptions includeArchived(final boolean includeArchived) {
		this.includeArchived = includeArchived;
		return this;
	}

	public void setIncludeArchived(final boolean includeArchived) {
		this.includeArchived = includeArchived;
	}

	public boolean getIncludeArchived() {
		return includeArchived;
	}
}
