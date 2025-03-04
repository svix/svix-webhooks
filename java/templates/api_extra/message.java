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


@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@NoArgsConstructor
@Getter
@Setter
// we use this because we need payload to be an object while the public `MessageIn.payload` is a
// string
private class MessageInInternal {
    @JsonProperty private ApplicationIn application;
    @JsonProperty private Set<String> channels;
    @JsonProperty private String eventId;
    @JsonProperty private String eventType;
    @JsonProperty private Object payload;
    @JsonProperty private Long payloadRetentionHours;
    @JsonProperty private Long payloadRetentionPeriod;
    @JsonProperty private Set<String> tags;
    @JsonProperty private Map<String, Object> transformationsParams;

    private MessageInInternal(MessageIn messageIn) {
        this.application = messageIn.getApplication();
        this.channels = messageIn.getChannels();
        this.eventId = messageIn.getEventId();
        this.eventType = messageIn.getEventType();
        this.payload = messageIn.getPayload();
        this.payloadRetentionHours = messageIn.getPayloadRetentionHours();
        this.payloadRetentionPeriod = messageIn.getPayloadRetentionPeriod();
        this.tags = messageIn.getTags();
        this.transformationsParams = messageIn.getTransformationsParams();
    }
}
