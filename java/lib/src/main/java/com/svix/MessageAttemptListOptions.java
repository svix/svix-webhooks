package com.svix;

import com.svix.models.MessageStatus;

public class MessageAttemptListOptions extends ListOptions {
    private MessageStatus status;

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

	@Override
	public MessageAttemptListOptions iterator(final String iterator) {
		return (MessageAttemptListOptions) super.iterator(iterator);
	}

	@Override
	public MessageAttemptListOptions limit(final Integer limit) {
		return (MessageAttemptListOptions) super.limit(limit);
	}
}
