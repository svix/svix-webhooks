// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointUpdate : BaseModel
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("url")]
        public required string Url { get; set; }

        [JsonProperty("version")]
        public ushort? Version { get; set; } = null;
    }
}
