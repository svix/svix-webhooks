// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointIn(
        string url,
        ushort? version = null,
        string? uid = null,
        string? secret = null,
        ushort? rateLimit = null,
        Dictionary<string, string>? metadata = null,
        List<string>? filterTypes = null,
        bool? disabled = null,
        string? description = null,
        List<string>? channels = null
    ) : BaseModel
    {
        [JsonPropertyName("channels")]
        public List<string>? Channels { get; set; } = channels;

        [JsonPropertyName("description")]
        public string? Description { get; set; } = description;

        [JsonPropertyName("disabled")]
        public bool? Disabled { get; set; } = disabled;

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; } = filterTypes;

        [JsonPropertyName("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = metadata;

        [JsonPropertyName("rateLimit")]
        public ushort? RateLimit { get; set; } = rateLimit;

        [JsonPropertyName("secret")]
        public string? Secret { get; set; } = secret;

        [JsonPropertyName("uid")]
        public string? Uid { get; set; } = uid;

        [JsonPropertyName("url")]
        public string Url { get; set; } = url;

        [JsonPropertyName("version")]
        public ushort? Version { get; set; } = version;
    }
}
