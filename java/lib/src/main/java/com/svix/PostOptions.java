package com.svix;

public class PostOptions {
    private String idempotencyKey;

	public PostOptions idempotencyKey(final String idempotencyKey) {
		this.idempotencyKey = idempotencyKey;
		return this;
	}

	public void setIdempotencyKey(final String idempotencyKey) {
		this.idempotencyKey = idempotencyKey;
	}

	public String getIdempotencyKey() {
		return idempotencyKey;
	}
}
