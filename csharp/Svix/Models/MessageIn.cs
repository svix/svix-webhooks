// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class MessageIn(
        Object payload,
        string eventType,
        Object? transformationsParams = null,
        List<string>? tags = null,
        long? payloadRetentionPeriod = null,
        long? payloadRetentionHours = null,
        string? eventId = null,
        List<string>? channels = null,
        ApplicationIn? application = null
    ) : BaseModel
    {
        [JsonPropertyName("application")]
        public ApplicationIn? Application { get; set; } = application;

        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; } = channels;

        [JsonPropertyName("eventId")]
        public string? EventId { get; set; } = eventId;

        [JsonPropertyName("eventType")]
        public string EventType { get; set; } = eventType;

        [JsonPropertyName("payload")]
        public Object Payload { get; set; } = payload;

        [JsonPropertyName("payloadRetentionHours")]
        public long? PayloadRetentionHours { get; set; } = payloadRetentionHours;

        [JsonPropertyName("payloadRetentionPeriod")]
        public long? PayloadRetentionPeriod { get; set; } = payloadRetentionPeriod;

        [JsonPropertyName("tags")]
        public List<string>? Tags { get; set; } = tags;

        [JsonPropertyName("transformationsParams")]
        public Object? TransformationsParams { get; set; } = transformationsParams;
    }
}
