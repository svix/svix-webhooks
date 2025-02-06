// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApplicationOut : BaseModel
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }
    }
}
