// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeOut : BaseModel
    {
        [JsonProperty("archived")]
        public bool? Archived { get; set; } = null;

        [JsonProperty("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("deprecated")]
        public required bool Deprecated { get; set; }

        [JsonProperty("description")]
        public required string Description { get; set; }

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("groupName")]
        public string? GroupName { get; set; } = null;

        [JsonProperty("name")]
        public required string Name { get; set; }

        [JsonProperty("schemas")]
        public Object? Schemas { get; set; } = null;

        [JsonProperty("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
