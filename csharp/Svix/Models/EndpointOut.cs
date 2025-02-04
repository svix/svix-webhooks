// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointOut : BaseModel
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("description")]
        public required string Description { get; set; }

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("metadata")]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt")]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("url")]
        public required string Url { get; set; }

        [JsonProperty("version")]
        public required int Version { get; set; }
    }
}
