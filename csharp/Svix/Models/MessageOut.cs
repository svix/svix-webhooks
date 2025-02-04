// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class MessageOut(
        DateTime timestamp,
        Object payload,
        string id,
        string eventType,
        List<string>? tags = null,
        string? eventId = null,
        List<string>? channels = null
    ) : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; } = channels;

        [JsonPropertyName("eventId")]
        public string? EventId { get; set; } = eventId;

        [JsonPropertyName("eventType")]
        public string EventType { get; set; } = eventType;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("payload")]
        public Object Payload { get; set; } = payload;

        [JsonPropertyName("tags")]
        public List<string>? Tags { get; set; } = tags;

        [JsonPropertyName("timestamp")]
        public DateTime Timestamp { get; set; } = timestamp;
    }
}
