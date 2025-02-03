// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class MessageOut : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; }

        [JsonPropertyName("eventId")]
        public string? EventId { get; set; }

        [JsonPropertyName("eventType")]
        public required string EventType { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("payload")]
        public required Object Payload { get; set; }

        [JsonPropertyName("tags")]
        public List<string>? Tags { get; set; }

        [JsonPropertyName("timestamp")]
        public required DateTime Timestamp { get; set; }
    }
}
