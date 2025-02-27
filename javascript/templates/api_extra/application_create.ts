/** Get the application with the UID from `applicationIn`, or create it if it doesn't exist yet. */
public getOrCreate(
  applicationIn: ApplicationIn,
  options?: ApplicationCreateOptions
): Promise<ApplicationOut> {
  const request = new SvixRequest(HttpMethod.POST, "/api/v1/app");

  request.setQueryParam("get_if_exists", true);
  request.setHeaderParam("idempotency-key", options?.idempotencyKey);
  request.setBody(ApplicationInSerializer._toJsonObject(applicationIn));

  return request.send(this.requestCtx, ApplicationOutSerializer._fromJsonObject);
}
