// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageAttemptOut : BaseModel
    {
        [JsonProperty("endpointId")]
        public required string EndpointId { get; set; }

        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("msg")]
        public MessageOut? Msg { get; set; } = null;

        [JsonProperty("msgId")]
        public required string MsgId { get; set; }

        [JsonProperty("response")]
        public required string Response { get; set; }

        [JsonProperty("responseDurationMs")]
        public required long ResponseDurationMs { get; set; }

        [JsonProperty("responseStatusCode")]
        public required short ResponseStatusCode { get; set; }

        [JsonProperty("status")]
        public required MessageStatus Status { get; set; }

        [JsonProperty("timestamp")]
        public required DateTime Timestamp { get; set; }

        [JsonProperty("triggerType")]
        public required MessageAttemptTriggerType TriggerType { get; set; }

        [JsonProperty("url")]
        public required string Url { get; set; }
    }
}
