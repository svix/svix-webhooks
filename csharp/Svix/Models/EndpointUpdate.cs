// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointUpdate : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; }

        [JsonPropertyName("description")]
        public string? Description { get; set; }

        [JsonPropertyName("disabled")]
        public bool? Disabled { get; set; }

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; }

        [JsonPropertyName("metadata")]
        public Dictionary<string, string>? Metadata { get; set; }

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; }

        [JsonPropertyName("uid")]
        public string? Uid { get; set; }

        [JsonPropertyName("url")]
        public required string Url { get; set; }

        [JsonPropertyName("version")]
        public ushort? Version { get; set; }
    }
}
