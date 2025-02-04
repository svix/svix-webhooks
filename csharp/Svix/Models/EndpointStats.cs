// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointStats(long success, long sending, long pending, long fail) : BaseModel
    {
        [JsonPropertyName("fail")]
        public long Fail { get; set; } = fail;

        [JsonPropertyName("pending")]
        public long Pending { get; set; } = pending;

        [JsonPropertyName("sending")]
        public long Sending { get; set; } = sending;

        [JsonPropertyName("success")]
        public long Success { get; set; } = success;
    }
}
