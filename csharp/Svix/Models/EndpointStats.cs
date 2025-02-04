// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointStats : BaseModel
    {
        [JsonProperty("fail")]
        public required long Fail { get; set; }

        [JsonProperty("pending")]
        public required long Pending { get; set; }

        [JsonProperty("sending")]
        public required long Sending { get; set; }

        [JsonProperty("success")]
        public required long Success { get; set; }
    }
}
