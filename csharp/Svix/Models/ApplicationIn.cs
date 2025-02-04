// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ApplicationIn(
        string name,
        string? uid = null,
        ushort? rateLimit = null,
        Dictionary<string, string>? metadata = null
    ) : BaseModel
    {
        [JsonPropertyName("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = metadata;

        [JsonPropertyName("name")]
        public string Name { get; set; } = name;

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; } = rateLimit;

        [JsonPropertyName("uid")]
        public string? Uid { get; set; } = uid;
    }
}
