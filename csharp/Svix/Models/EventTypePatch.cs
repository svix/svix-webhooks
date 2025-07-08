// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypePatch
    {
        [JsonProperty("archived")]
        public bool? Archived { get; set; } = null;

        public bool ShouldSerializeArchived() => Archived != null;

        [JsonProperty("deprecated")]
        public bool? Deprecated { get; set; } = null;

        public bool ShouldSerializeDeprecated() => Deprecated != null;

        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        public bool ShouldSerializeDescription() => Description != null;

        [JsonProperty("featureFlag")]
        public MaybeUnset<string?> FeatureFlag { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeFeatureFlag() => !FeatureFlag.IsUnset;

        [JsonProperty("featureFlags")]
        public MaybeUnset<List<string>?> FeatureFlags { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeFeatureFlags() => !FeatureFlags.IsUnset;

        [JsonProperty("groupName")]
        public MaybeUnset<string?> GroupName { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeGroupName() => !GroupName.IsUnset;

        [JsonProperty("schemas")]
        public MaybeUnset<Object?> Schemas { get; set; } = MaybeUnset<Object?>.Unset();

        public bool ShouldSerializeSchemas() => !Schemas.IsUnset;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventTypePatch {\n");
            sb.Append("  Archived: ").Append(Archived).Append('\n');
            sb.Append("  Deprecated: ").Append(Deprecated).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlag: ").Append(FeatureFlag).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  GroupName: ").Append(GroupName).Append('\n');
            sb.Append("  Schemas: ").Append(Schemas).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
