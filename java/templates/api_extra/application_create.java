/** Create a new application. */
public ApplicationOut getOrCreate(final ApplicationIn applicationIn)
        throws IOException, ApiException {
    return this.getOrCreate(applicationIn, new ApplicationCreateOptions());
}

/** Create a new application. */
public ApplicationOut getOrCreate(
        final ApplicationIn applicationIn, final ApplicationCreateOptions options)
        throws IOException, ApiException {
    HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/app");
    url.addQueryParameter("get_if_exists","true");
    Map<String, String> headers = new HashMap<>();
    if (options.idempotencyKey != null) {
        headers.put("idempotency-key", options.idempotencyKey);
    }
    return this.client.executeRequest(
            "POST", url.build(), Headers.of(headers), applicationIn, ApplicationOut.class);
}
