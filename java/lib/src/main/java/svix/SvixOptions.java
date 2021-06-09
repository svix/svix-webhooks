package com.svix;

public class SvixOptions {
	private final String DEFAULT_URL = "https://api.svix.com";

	private boolean debug = false;
	private String debugUrl = DEFAULT_URL;

	public SvixOptions() {
	}

	SvixOptions setDebug(boolean debug) {
		this.debug = debug;
		return this;
	}

	SvixOptions setUrl(String debugUrl) {
		this.debugUrl = debugUrl;
		return this;
	}

	public boolean getDebug() {
		return debug;
	}

	public String getUrl() {
		return debugUrl;
	}
}
