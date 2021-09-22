package com.svix;

import java.util.List;
import org.threeten.bp.OffsetDateTime;
import com.svix.models.MessageStatus;

public class MessageAttemptListOptions extends ListOptions {
    private MessageStatus status;
	private List<String> eventTypes;
	private OffsetDateTime before;

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
}
