package com.svix;

public final class SvixOptions {
	private static final String DEFAULT_URL = "https://api.svix.com";

	private boolean debug = false;
	private String debugUrl = DEFAULT_URL;

	public SvixOptions() {
	}

	public SvixOptions debug(final boolean debug) {
		this.debug = debug;
		return this;
	}

	public SvixOptions debugUrl(final String debugUrl) {
		this.debugUrl = debugUrl;
		return this;
	}

	public void setDebug(final boolean debug) {
		this.debug = debug;
	}

	public void setDebugUrl(final String debugUrl) {
		this.debugUrl = debugUrl;
	}

	public boolean getDebug() {
		return debug;
	}

	public String getDebugUrl() {
		return debugUrl;
	}
}
