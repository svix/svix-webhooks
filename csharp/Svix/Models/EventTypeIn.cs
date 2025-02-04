// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeIn : BaseModel
    {
        [JsonProperty("archived")]
        public bool? Archived { get; set; } = null;

        [JsonProperty("deprecated")]
        public bool? Deprecated { get; set; } = null;

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
    }
}
