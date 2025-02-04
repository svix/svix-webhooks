// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointOut(
        int version,
        string url,
        DateTime updatedAt,
        Dictionary<string, string> metadata,
        string id,
        string description,
        DateTime createdAt,
        string? uid = null,
        ushort? rateLimit = null,
        List<string>? filterTypes = null,
        bool? disabled = null,
        List<string>? channels = null
    ) : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; } = channels;

        [JsonPropertyName("createdAt")]
        public DateTime CreatedAt { get; set; } = createdAt;

        [JsonPropertyName("description")]
        public string Description { get; set; } = description;

        [JsonPropertyName("disabled")]
        public bool? Disabled { get; set; } = disabled;

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; } = filterTypes;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("metadata")]
        public Dictionary<string, string> Metadata { get; set; } = metadata;

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; } = rateLimit;

        [JsonPropertyName("uid")]
        public string? Uid { get; set; } = uid;

        [JsonPropertyName("updatedAt")]
        public DateTime UpdatedAt { get; set; } = updatedAt;

        [JsonPropertyName("url")]
        public string Url { get; set; } = url;

        [JsonPropertyName("version")]
        public int Version { get; set; } = version;
    }
}
