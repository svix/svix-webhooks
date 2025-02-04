// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypePatch : BaseModel
    {
        [JsonProperty("archived")]
        public bool? Archived { get; set; } = null;

        [JsonProperty("deprecated")]
        public bool? Deprecated { get; set; } = null;

        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("featureFlag")]
        public MaybeUnset<string?> FeatureFlag { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeFeatureFlag() => !FeatureFlag.IsUnset;

        [JsonProperty("groupName")]
        public MaybeUnset<string?> GroupName { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeGroupName() => !GroupName.IsUnset;

        [JsonProperty("schemas")]
        public MaybeUnset<Object?> Schemas { get; set; } = MaybeUnset<Object?>.Unset();

        public bool ShouldSerializeSchemas() => !Schemas.IsUnset;
    }
}
