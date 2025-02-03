// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    /// <summary>
    /// A model containing information on a given message plus additional fields on the last attempt for that message.
    /// <summary>

    public class EndpointMessageOut : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; }

        [JsonPropertyName("eventId")]
        public string? EventId { get; set; }

        [JsonPropertyName("eventType")]
        public required string EventType { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("nextAttempt")]
        public DateTime? NextAttempt { get; set; }

        [JsonPropertyName("payload")]
        public required Object Payload { get; set; }

        [JsonPropertyName("status")]
        public required MessageStatus Status { get; set; }

        [JsonPropertyName("tags")]
        public List<string>? Tags { get; set; }

        [JsonPropertyName("timestamp")]
        public required DateTime Timestamp { get; set; }
    }
}
