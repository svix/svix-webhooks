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
}
