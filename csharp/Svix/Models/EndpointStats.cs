// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointStats : BaseModel
    {
        [JsonPropertyName("fail")]
        public required long Fail { get; set; }

        [JsonPropertyName("pending")]
        public required long Pending { get; set; }

        [JsonPropertyName("sending")]
        public required long Sending { get; set; }

        [JsonPropertyName("success")]
        public required long Success { get; set; }
    }
}
