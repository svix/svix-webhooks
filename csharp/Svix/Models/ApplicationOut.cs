// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ApplicationOut : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("metadata")]
        public required Dictionary<string, string> Metadata { get; set; }

        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; }

        [JsonPropertyName("uid")]
        public string? Uid { get; set; }

        [JsonPropertyName("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
