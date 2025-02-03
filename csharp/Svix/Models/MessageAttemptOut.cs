// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class MessageAttemptOut : BaseModel
    {
        [JsonPropertyName("endpointId")]
        public required string EndpointId { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("msg")]
        public MessageOut? Msg { get; set; }

        [JsonPropertyName("msgId")]
        public required string MsgId { get; set; }

        [JsonPropertyName("response")]
        public required string Response { get; set; }

        [JsonPropertyName("responseDurationMs")]
        public required long ResponseDurationMs { get; set; }

        [JsonPropertyName("responseStatusCode")]
        public required short ResponseStatusCode { get; set; }

        [JsonPropertyName("status")]
        public required MessageStatus Status { get; set; }

        [JsonPropertyName("timestamp")]
        public required DateTime Timestamp { get; set; }

        [JsonPropertyName("triggerType")]
        public required MessageAttemptTriggerType TriggerType { get; set; }

        [JsonPropertyName("url")]
        public required string Url { get; set; }
    }
}
