// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageOut : BaseModel
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventId")]
        public string? EventId { get; set; } = null;

        [JsonProperty("eventType")]
        public required string EventType { get; set; }

        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("payload")]
        public required Object Payload { get; set; }

        [JsonProperty("tags")]
        public List<string>? Tags { get; set; } = null;

        [JsonProperty("timestamp")]
        public required DateTime Timestamp { get; set; }
    }
}
