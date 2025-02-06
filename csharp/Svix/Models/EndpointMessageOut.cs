// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// A model containing information on a given message plus additional fields on the last attempt for that message.
    /// <summary>

    public class EndpointMessageOut : BaseModel
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventId")]
        public string? EventId { get; set; } = null;

        [JsonProperty("eventType", Required = Required.Always)]
        public required string EventType { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("nextAttempt")]
        public DateTime? NextAttempt { get; set; } = null;

        [JsonProperty("payload", Required = Required.Always)]
        public required Object Payload { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required MessageStatus Status { get; set; }

        [JsonProperty("tags")]
        public List<string>? Tags { get; set; } = null;

        [JsonProperty("timestamp", Required = Required.Always)]
        public required DateTime Timestamp { get; set; }
    }
}
