package svix;

public class SvixOptions {
	private final boolean debug;
	private final String testUrl;

	public SvixOptions(boolean debug, String testUrl) {
		this.debug = debug;
		this.testUrl = testUrl;
	}

	public boolean getDebug() {
		return debug;
	}

	public String getUrl() {
		return testUrl;
	}
}
