/** Get or create an application. */
suspend fun getOrCreate(
    applicationIn: ApplicationIn,
    options: ApplicationCreateOptions = ApplicationCreateOptions(),
): ApplicationOut {
    val url =
        client.newUrlBuilder().encodedPath("/api/v1/app").addQueryParameter("get_if_exists", "true")
    var headers = Headers.Builder()
    options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

    return client.executeRequest<ApplicationIn, ApplicationOut>(
        "POST",
        url.build(),
        headers = headers.build(),
        reqBody = applicationIn,
    )
}
