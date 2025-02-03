// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class MessageIn : BaseModel
    {
        [JsonPropertyName("application")]
        public ApplicationIn? Application { get; set; }

        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; }

        [JsonPropertyName("eventId")]
        public string? EventId { get; set; }

        [JsonPropertyName("eventType")]
        public required string EventType { get; set; }

        [JsonPropertyName("payload")]
        public required Object Payload { get; set; }

        [JsonPropertyName("payloadRetentionHours")]
        public long? PayloadRetentionHours { get; set; }

        [JsonPropertyName("payloadRetentionPeriod")]
        public long? PayloadRetentionPeriod { get; set; }

        [JsonPropertyName("tags")]
        public List<string>? Tags { get; set; }

        [JsonPropertyName("transformationsParams")]
        public Object? TransformationsParams { get; set; }
    }
}
