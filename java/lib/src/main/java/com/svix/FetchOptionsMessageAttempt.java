package com.svix;

import com.svix.models.MessageStatus;


public final class FetchOptionsMessageAttempt extends FetchOptions {

    private MessageStatus status;

	public FetchOptionsMessageAttempt() {
        super();
	}

	public FetchOptionsMessageAttempt messageStatus(final MessageStatus status) {
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
