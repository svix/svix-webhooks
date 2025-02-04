// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    /// <summary>
    /// A model containing information on a given message plus additional fields on the last attempt for that message.
    /// <summary>

    public class EndpointMessageOut(
        DateTime timestamp,
        MessageStatus status,
        Object payload,
        string id,
        string eventType,
        List<string>? tags = null,
        DateTime? nextAttempt = null,
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

        [JsonPropertyName("nextAttempt")]
        public DateTime? NextAttempt { get; set; } = nextAttempt;

        [JsonPropertyName("payload")]
        public Object Payload { get; set; } = payload;

        [JsonPropertyName("status")]
        public MessageStatus Status { get; set; } = status;

        [JsonPropertyName("tags")]
        public List<string>? Tags { get; set; } = tags;

        [JsonPropertyName("timestamp")]
        public DateTime Timestamp { get; set; } = timestamp;
    }
}
