/**
 * Creates a MessageIn with a raw string payload.
 *
 * <p>The payload is not normalized on the server. Normally, payloads are required to be JSON,
 * and Svix will minify the payload before sending the webhooks (for example, by removing
 * extraneous whitespace or unnecessarily escaped characters in strings). With this function,
 * the payload will be sent "as is", without any minification or other processing.
 *
 * @param payload Serialized message payload
 */
public static MessageIn messageInRaw(final String payload) {
    MessageIn msg = new MessageIn();
    msg.setPayload("");
    msg.setTransformationsParams(Collections.singletonMap("rawPayload", payload));
    return msg;
}

/**
 * Creates a MessageIn with a raw string payload.
 *
 * <p>This overload is intended for non-JSON payloads.
 *
 * @param payload Serialized message payload
 * @param contentType The value to use for the Content-Type header of the webhook sent by Svix
 */
public static MessageIn messageInRaw(final String payload, final String contentType) {
    HashMap<String, Object> trParam = new HashMap<>();
    trParam.put("rawPayload", payload);
    trParam.put("headers", Collections.singletonMap("content-type", contentType));
    MessageIn msg = new MessageIn();
    msg.setPayload("");
    msg.setTransformationsParams(trParam);
    return msg;
}
