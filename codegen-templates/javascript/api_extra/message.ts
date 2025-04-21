/**
 * Creates a `MessageIn` with a raw string payload.
 *
 * The payload is not normalized on the server. Normally, payloads are
 * required to be JSON, and Svix will minify the payload before sending the
 * webhooks (for example, by removing extraneous whitespace or unnecessarily
 * escaped characters in strings). With this function, the payload will be
 * sent "as is", without any minification or other processing.
 *
 * @param payload Serialized message payload
 * @param contentType The value to use for the Content-Type header of the webhook sent by Svix, overwriting the default of `application/json` if specified
 *
 * See the class documentation for details about the other parameters.
 */
export function messageInRaw(
  eventType: string,
  payload: string,
  contentType?: string
): MessageIn {
  const headers = contentType ? { "content-type": contentType } : undefined;

  return {
    eventType,
    payload: {},
    transformationsParams: {
      rawPayload: payload,
      headers,
    },
  };
}
