// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class MessageAttemptOut(
        string url,
        MessageAttemptTriggerType triggerType,
        DateTime timestamp,
        MessageStatus status,
        short responseStatusCode,
        long responseDurationMs,
        string response,
        string msgId,
        string id,
        string endpointId,
        MessageOut? msg = null
    ) : BaseModel
    {
        [JsonPropertyName("endpointId")]
        public string EndpointId { get; set; } = endpointId;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("msg")]
        public MessageOut? Msg { get; set; } = msg;

        [JsonPropertyName("msgId")]
        public string MsgId { get; set; } = msgId;

        [JsonPropertyName("response")]
        public string Response { get; set; } = response;

        [JsonPropertyName("responseDurationMs")]
        public long ResponseDurationMs { get; set; } = responseDurationMs;

        [JsonPropertyName("responseStatusCode")]
        public short ResponseStatusCode { get; set; } = responseStatusCode;

        [JsonPropertyName("status")]
        public MessageStatus Status { get; set; } = status;

        [JsonPropertyName("timestamp")]
        public DateTime Timestamp { get; set; } = timestamp;

        [JsonPropertyName("triggerType")]
        public MessageAttemptTriggerType TriggerType { get; set; } = triggerType;

        [JsonPropertyName("url")]
        public string Url { get; set; } = url;
    }
}
