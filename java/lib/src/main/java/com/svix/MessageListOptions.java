package com.svix;

import java.util.List;
import org.threeten.bp.OffsetDateTime;

public class MessageListOptions extends ListOptions {
    private List<String> eventTypes;
	private OffsetDateTime before;

    public MessageListOptions eventTypes(final List<String> eventTypes) {
		this.eventTypes = eventTypes;
		return this;
	}

	public void setEventTypes(final List<String> eventTypes) {
		this.eventTypes = eventTypes;
	}

	public List<String> getEventTypes() {
		return eventTypes;
	}

	public MessageListOptions before(final OffsetDateTime before) {
		this.before = before;
		return this;
	}

	public void setBefore(final OffsetDateTime before) {
		this.before = before;
	}

	public OffsetDateTime getBefore() {
		return before;
	}
}
