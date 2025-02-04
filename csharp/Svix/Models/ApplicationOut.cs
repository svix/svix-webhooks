// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ApplicationOut(
        DateTime updatedAt,
        string name,
        Dictionary<string, string> metadata,
        string id,
        DateTime createdAt,
        string? uid = null,
        ushort? rateLimit = null
    ) : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public DateTime CreatedAt { get; set; } = createdAt;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("metadata")]
        public Dictionary<string, string> Metadata { get; set; } = metadata;

        [JsonPropertyName("name")]
        public string Name { get; set; } = name;

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; } = rateLimit;

        [JsonPropertyName("uid")]
        public string? Uid { get; set; } = uid;

        [JsonPropertyName("updatedAt")]
        public DateTime UpdatedAt { get; set; } = updatedAt;
    }
}
