// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointStats : BaseModel
    {
        [JsonProperty("fail", Required = Required.Always)]
        public required long Fail { get; set; }

        [JsonProperty("pending", Required = Required.Always)]
        public required long Pending { get; set; }

        [JsonProperty("sending", Required = Required.Always)]
        public required long Sending { get; set; }

        [JsonProperty("success", Required = Required.Always)]
        public required long Success { get; set; }
    }
}
