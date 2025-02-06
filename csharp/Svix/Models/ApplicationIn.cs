// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApplicationIn : BaseModel
    {
        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;
    }
}
