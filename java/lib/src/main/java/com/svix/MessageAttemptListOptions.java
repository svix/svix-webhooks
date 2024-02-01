package com.svix;

import java.util.List;
import java.time.OffsetDateTime;

import com.svix.models.MessageStatus;
import com.svix.models.StatusCodeClass;

public class MessageAttemptListOptions extends ListOptions {
	private MessageStatus status;
	private List<String> eventTypes;
	private OffsetDateTime before;
	private OffsetDateTime after;
	private StatusCodeClass statusCodeClass;
	private String channel;
	private String tag;
	private Boolean withContent;
	private String endpointId;
	private Boolean withMsg;

	public MessageAttemptListOptions() {
		super();
	}

	public MessageAttemptListOptions messageStatus(final MessageStatus status) {
		this.status = status;
		return this;
	}

	public void setMessageStatus(final MessageStatus status) {
		this.status = status;
	}

	public MessageStatus getMessageStatus() {
		return status;
	}

	public MessageAttemptListOptions eventTypes(final List<String> eventTypes) {
		this.eventTypes = eventTypes;
		return this;
	}

	public void setEventTypes(final List<String> eventTypes) {
		this.eventTypes = eventTypes;
	}

	public List<String> getEventTypes() {
		return eventTypes;
	}

	public MessageAttemptListOptions channel(final String channel) {
		this.channel = channel;
		return this;
	}

	public void setChannel(final String channel) {
		this.channel = channel;
	}

	public String getChannel() {
		return channel;
	}

	public MessageAttemptListOptions before(final OffsetDateTime before) {
		this.before = before;
		return this;
	}

	public void setBefore(final OffsetDateTime before) {
		this.before = before;
	}

	public OffsetDateTime getBefore() {
		return before;
	}

	public MessageAttemptListOptions after(final OffsetDateTime after) {
		this.after = after;
		return this;
	}

	public void setAfter(final OffsetDateTime after) {
		this.after = after;
	}

	public OffsetDateTime getAfter() {
		return after;
	}

	public MessageAttemptListOptions statusCodeClass(final StatusCodeClass statusCodeClass) {
		this.statusCodeClass = statusCodeClass;
		return this;
	}

	public void setStatusCodeClass(final StatusCodeClass statusCodeClass) {
		this.statusCodeClass = statusCodeClass;
	}

	public StatusCodeClass getStatusCodeClass() {
		return statusCodeClass;
	}

	public Boolean getWithContent() {
		return withContent;
	}

	public void setWithContent(final Boolean withContent) {
		this.withContent = withContent;
	}

	public MessageAttemptListOptions withContent(final Boolean withContent) {
		this.withContent = withContent;
		return this;
	}

	public String getEndpointId() {
		return endpointId;
	}

	public void setEndpointId(final String endpointId) {
		this.endpointId = endpointId;
	}

	public MessageAttemptListOptions endpointId(final String endpointId) {
		this.endpointId = endpointId;
		return this;
	}

	public Boolean getWithMsg() {
		return withMsg;
	}

	public void setWithMsg(final Boolean withMsg) {
		this.withMsg = withMsg;
	}

	public MessageAttemptListOptions withMsg(final Boolean withMsg) {
		this.withMsg = withMsg;
		return this;
	}

	public String getTag() {
		return tag;
	}

	public void setTag(final String tag) {
		this.tag = tag;
	}

	public MessageAttemptListOptions tag(final String tag) {
		this.tag = tag;
		return this;
	}
}
