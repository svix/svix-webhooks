// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypeOut(
        DateTime updatedAt,
        string name,
        string description,
        bool deprecated,
        DateTime createdAt,
        Object? schemas = null,
        string? groupName = null,
        string? featureFlag = null,
        bool? archived = null
    ) : BaseModel
    {
        [JsonPropertyName("archived")]
        public bool? Archived { get; set; } = archived;

        [JsonPropertyName("createdAt")]
        public DateTime CreatedAt { get; set; } = createdAt;

        [JsonPropertyName("deprecated")]
        public bool Deprecated { get; set; } = deprecated;

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

        [JsonPropertyName("updatedAt")]
        public DateTime UpdatedAt { get; set; } = updatedAt;
    }
}
