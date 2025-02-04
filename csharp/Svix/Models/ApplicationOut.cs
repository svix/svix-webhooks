// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApplicationOut : BaseModel
    {
        [JsonProperty("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("metadata")]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("name")]
        public required string Name { get; set; }

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
