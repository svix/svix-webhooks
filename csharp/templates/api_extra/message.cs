/// <summary>Creates a [MessageIn] with a raw string payload.
/// <para>
/// The payload is not normalized on the server. Normally, payloads are
/// required to be JSON, and Svix will minify the payload before sending the
/// webhooks (for example, by removing extraneous whitespace or unnecessarily
/// escaped characters in strings). With this function, the payload will be
/// sent "as is", without any minification or other processing.
/// </para>
/// </summary>
/// <param name="payload">Serialized message payload</param>
/// <param name="contentType">The `content-type` header of the webhook sent by Svix,
/// overwriting the default of `application/json` if specified</param>
public static MessageIn messageInRaw(
    string eventType,
    string payload,
    string? contentType = null,
    ApplicationIn? application = null,
    List<string>? channels = null,
    string? eventId = null,
    long? payloadRetentionHours = null,
    long? payloadRetentionPeriod = null,
    List<string>? tags = null,
    Dictionary<string, Object>? transformationsParams = null
)
{
    if (transformationsParams == null)
    {
        transformationsParams = new Dictionary<string, object>();
    }

    transformationsParams["rawPayload"] = payload;
    if (contentType != null)
    {
        transformationsParams["headers"] = new Dictionary<string, string>
        {
            ["content-type"] = contentType,
        };
    }

    return new MessageIn
    {
        EventType = eventType,
        Payload = new { },
        Application = application,
        Channels = channels,
        EventId = eventId,
        PayloadRetentionHours = payloadRetentionHours,
        PayloadRetentionPeriod = payloadRetentionPeriod,
        Tags = tags,
        TransformationsParams = transformationsParams,
    };
}
