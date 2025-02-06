// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeFromOpenApi : BaseModel
    {
        [JsonProperty("deprecated", Required = Required.Always)]
        public required bool Deprecated { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public required string Description { get; set; }

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("groupName")]
        public string? GroupName { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("schemas")]
        public Object? Schemas { get; set; } = null;
    }
}
