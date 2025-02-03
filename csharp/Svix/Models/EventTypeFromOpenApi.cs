// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypeFromOpenApi : BaseModel
    {
        [JsonPropertyName("deprecated")]
        public required bool Deprecated { get; set; }

        [JsonPropertyName("description")]
        public required string Description { get; set; }

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; }

        [JsonPropertyName("groupName")]
        public string? GroupName { get; set; }

        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("schemas")]
        public Object? Schemas { get; set; }
    }
}
