package com.svix.exceptions;

import java.util.Map;
import java.util.List;

public class WebhookVerificationException extends Exception {
	private int code = 0;
	private Map<String, List<String>> responseHeaders = null;
	private String responseBody = null;

	public WebhookVerificationException() {
	}

	public WebhookVerificationException(final Throwable throwable) {
		super(throwable);
	}

	public WebhookVerificationException(final String message) {
		super(message);
	}

	public WebhookVerificationException(final String message, final Throwable throwable, final int code,
	    final Map<String, List<String>> responseHeaders, final String responseBody) {
		super(message, throwable);
		this.code = code;
		this.responseHeaders = responseHeaders;
		this.responseBody = responseBody;
	}

	public WebhookVerificationException(final String message, final int code, final Map<String, List<String>> responseHeaders,
	    final String responseBody) {
		this(message, (Throwable) null, code, responseHeaders, responseBody);
	}

	public WebhookVerificationException(final String message, final Throwable throwable, final int code,
	    final Map<String, List<String>> responseHeaders) {
		this(message, throwable, code, responseHeaders, null);
	}

	public WebhookVerificationException(final int code, final Map<String, List<String>> responseHeaders, final String responseBody) {
		this((String) null, (Throwable) null, code, responseHeaders, responseBody);
	}

	public WebhookVerificationException(final int code, final String message) {
		super(message);
		this.code = code;
	}

	public WebhookVerificationException(final int code, final String message, final Map<String, List<String>> responseHeaders,
	    final String responseBody) {
		this(code, message);
		this.responseHeaders = responseHeaders;
		this.responseBody = responseBody;
	}

	/**
	 * Get the HTTP status code.
	 *
	 * @return HTTP status code
	 */
	public int getCode() {
		return code;
	}

	/**
	 * Get the HTTP response headers.
	 *
	 * @return A map of list of string
	 */
	public Map<String, List<String>> getResponseHeaders() {
		return responseHeaders;
	}

	/**
	 * Get the HTTP response body.
	 *
	 * @return Response body in the form of string
	 */
	public String getResponseBody() {
		return responseBody;
	}
}
