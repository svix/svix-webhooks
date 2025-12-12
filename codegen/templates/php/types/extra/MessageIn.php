/**
    * Creates a MessageIn with a raw string payload.
    *
    * The payload is not normalized on the server. Normally, payloads are required
    * to be JSON, and Svix will minify the payload before sending the webhook
    * (for example, by removing extraneous whitespace or unnecessarily escaped
    * characters in strings). With this function, the payload will be sent
    * "as is", without  any minification or other processing.
    *
    * @param string      $eventType   The event type's name Example: `user.signup`.
    * @param string      $payload     Serialized message payload.
    * @param string|null $contentType The value to use for the Content-Type header of the
    *                                 webhook sent by Svix, overwriting the default of `application/json`
    *                                 if specified.
    */
public static function createRaw(
    string $eventType,
    string $payload,
    ?string $contentType = null,
): self {
    $transformationsParams = [
        'rawPayload' => $payload,
    ];
    if ($contentType !== null) {
        $transformationsParams['headers'] = ['content-type' => $contentType];
    }

    return new self(
        application: null,
        channels: null,
        deliverAt: null,
        eventId: null,
        eventType: $eventType,
        payload: [],
        payloadRetentionHours: null,
        payloadRetentionPeriod: null,
        tags: null,
        transformationsParams: $transformationsParams,
        setFields: ['eventType' => true, 'payload' => true, 'transformationsParams' => true]
    );
}
