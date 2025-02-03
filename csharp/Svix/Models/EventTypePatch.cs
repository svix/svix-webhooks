// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypePatch : BaseModel
    {
        [JsonPropertyName("archived")]
        public bool? Archived { get; set; }

        [JsonPropertyName("deprecated")]
        public bool? Deprecated { get; set; }

        [JsonPropertyName("description")]
        public string? Description { get; set; }

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; }

        [JsonPropertyName("groupName")]
        public string? GroupName { get; set; }

        [JsonPropertyName("schemas")]
        public Object? Schemas { get; set; }
    }
}
