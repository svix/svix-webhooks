// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class OperationalWebhookEndpointOut : BaseModel
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public required string Description { get; set; }

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }
    }
}
