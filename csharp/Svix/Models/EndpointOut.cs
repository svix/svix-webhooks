// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointOut : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; }

        [JsonPropertyName("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonPropertyName("description")]
        public required string Description { get; set; }

        [JsonPropertyName("disabled")]
        public bool? Disabled { get; set; }

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("metadata")]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; }

        [JsonPropertyName("uid")]
        public string? Uid { get; set; }

        [JsonPropertyName("updatedAt")]
        public required DateTime UpdatedAt { get; set; }

        [JsonPropertyName("url")]
        public required string Url { get; set; }

        [JsonPropertyName("version")]
        public required int Version { get; set; }
    }
}
