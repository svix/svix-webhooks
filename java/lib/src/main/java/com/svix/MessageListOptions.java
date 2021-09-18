package com.svix;

import java.util.List;

public class MessageListOptions extends ListOptions {
    private List<String> eventTypes;

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

	@Override
	public MessageListOptions iterator(final String iterator) {
		return (MessageListOptions) super.iterator(iterator);
	}

	@Override
	public MessageListOptions limit(final Integer limit) {
		return (MessageListOptions) super.limit(limit);
	}
}
