// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypeIn(
        string name,
        string description,
        Object? schemas = null,
        string? groupName = null,
        string? featureFlag = null,
        bool? deprecated = null,
        bool? archived = null
    ) : BaseModel
    {
        [JsonPropertyName("archived")]
        public bool? Archived { get; set; } = archived;

        [JsonPropertyName("deprecated")]
        public bool? Deprecated { get; set; } = deprecated;

        [JsonPropertyName("description")]
        public string Description { get; set; } = description;

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; } = featureFlag;

        [JsonPropertyName("groupName")]
        public string? GroupName { get; set; } = groupName;

        [JsonPropertyName("name")]
        public string Name { get; set; } = name;

        [JsonPropertyName("schemas")]
        public Object? Schemas { get; set; } = schemas;
    }
}
