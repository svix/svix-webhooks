// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypePatch : BaseModel
    {
        [JsonProperty("archived")]
        public MaybeUnset<bool?> Archived { get; set; } = MaybeUnset<bool?>.Unset();

        public bool ShouldSerializeArchived() => !Archived.IsUnset;

        [JsonProperty("deprecated")]
        public MaybeUnset<bool?> Deprecated { get; set; } = MaybeUnset<bool?>.Unset();

        public bool ShouldSerializeDeprecated() => !Deprecated.IsUnset;

        [JsonProperty("description")]
        public MaybeUnset<string?> Description { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeDescription() => !Description.IsUnset;

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
