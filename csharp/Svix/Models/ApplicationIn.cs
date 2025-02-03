// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ApplicationIn : BaseModel
    {
        [JsonPropertyName("metadata")]
        public Dictionary<string, string>? Metadata { get; set; }

        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; }

        [JsonPropertyName("uid")]
        public string? Uid { get; set; }
    }
}
