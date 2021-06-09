package com.svix;

import com.svix.models.MessageStatus;


public final class FetchOptionsMessageAttempt extends FetchOptions {

    private MessageStatus status;

	public FetchOptionsMessageAttempt() {
        super();
	}

	public FetchOptionsMessageAttempt setMessageStatus(final MessageStatus status) {
		this.status = status;
		return this;
	}

	public MessageStatus getMessageStatus() {
		return status;
	}
}
