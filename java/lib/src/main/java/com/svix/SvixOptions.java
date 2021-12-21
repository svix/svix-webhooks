package com.svix;

public final class SvixOptions {
	private static final String DEFAULT_URL = "https://api.svix.com";

	private boolean debug = false;
	private String serverUrl = DEFAULT_URL;

	public SvixOptions() {
	}

	public SvixOptions debug(final boolean debug) {
		this.debug = debug;
		return this;
	}

	public SvixOptions serverUrl(final String serverUrl) {
		this.serverUrl = serverUrl;
		return this;
	}

	public void setDebug(final boolean debug) {
		this.debug = debug;
	}

	public void setServerUrl(final String serverUrl) {
		this.serverUrl = serverUrl;
	}

	public boolean getDebug() {
		return debug;
	}

	public String getServerUrl() {
		return serverUrl;
	}
}
