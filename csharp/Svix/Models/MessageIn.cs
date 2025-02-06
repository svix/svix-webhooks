// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageIn : BaseModel
    {
        [JsonProperty("application")]
        public ApplicationIn? Application { get; set; } = null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventId")]
        public string? EventId { get; set; } = null;

        [JsonProperty("eventType", Required = Required.Always)]
        public required string EventType { get; set; }

        [JsonProperty("payload", Required = Required.Always)]
        public required Object Payload { get; set; }

        [JsonProperty("payloadRetentionHours")]
        public long? PayloadRetentionHours { get; set; } = null;

        [JsonProperty("payloadRetentionPeriod")]
        public long? PayloadRetentionPeriod { get; set; } = null;

        [JsonProperty("tags")]
        public List<string>? Tags { get; set; } = null;

        [JsonProperty("transformationsParams")]
        public Object? TransformationsParams { get; set; } = null;
    }
}
