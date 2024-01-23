package com.svix;

import java.util.List;
import java.time.OffsetDateTime;

public class MessageListOptions extends ListOptions {
	private List<String> eventTypes;
	private OffsetDateTime before;
	private OffsetDateTime after;
	private String channel;
	private Boolean withContent;
	private String tag;

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

	public MessageListOptions channel(final String channel) {
		this.channel = channel;
		return this;
	}

	public void setChannel(final String channel) {
		this.channel = channel;
	}

	public String getChannel() {
		return channel;
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

	public MessageListOptions after(final OffsetDateTime after) {
		this.after = after;
		return this;
	}

	public void setAfter(final OffsetDateTime after) {
		this.after = after;
	}

	public OffsetDateTime getAfter() {
		return after;
	}

	public Boolean getWithContent() {
		return withContent;
	}

	public void setWithContent(final Boolean withContent) {
		this.withContent = withContent;
	}

	public MessageListOptions withContent(final Boolean withContent) {
		this.withContent = withContent;
		return this;
	}

	public String getTag() {
		return tag;
	}

	public MessageListOptions tag(final String tag) {
		this.tag = tag;
		return this;
	}

	public void setTag(final String tag) {
		this.tag = tag;
	}
}
