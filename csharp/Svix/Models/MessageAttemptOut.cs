// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageAttemptOut : BaseModel
    {
        [JsonProperty("endpointId", Required = Required.Always)]
        public required string EndpointId { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("msg")]
        public MessageOut? Msg { get; set; } = null;

        [JsonProperty("msgId", Required = Required.Always)]
        public required string MsgId { get; set; }

        [JsonProperty("response", Required = Required.Always)]
        public required string Response { get; set; }

        [JsonProperty("responseDurationMs", Required = Required.Always)]
        public required long ResponseDurationMs { get; set; }

        [JsonProperty("responseStatusCode", Required = Required.Always)]
        public required short ResponseStatusCode { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required MessageStatus Status { get; set; }

        [JsonProperty("timestamp", Required = Required.Always)]
        public required DateTime Timestamp { get; set; }

        [JsonProperty("triggerType", Required = Required.Always)]
        public required MessageAttemptTriggerType TriggerType { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }
    }
}
