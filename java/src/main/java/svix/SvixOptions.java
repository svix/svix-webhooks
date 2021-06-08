package svix;

public class SvixOptions {
	private final String DEFAULT_URL = "https://api.svix.com";

	private final boolean debug;
	private final String debugUrl;

	public SvixOptions() {
		this(false, null);
	}

	public SvixOptions(boolean debug) {
		this(debug, null);
	}

	public SvixOptions(String debugUrl) {
		this(false, debugUrl);
	}

	public SvixOptions(boolean debug, String debugUrl) {
		this.debug = debug;
		this.debugUrl = debugUrl == null ? DEFAULT_URL : debugUrl;
	}

	public boolean getDebug() {
		return debug;
	}

	public String getUrl() {
		return debugUrl;
	}
}
