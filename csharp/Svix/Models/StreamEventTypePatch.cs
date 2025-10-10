// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class StreamEventTypePatch
    {
        [JsonProperty("archived")]
        public bool? Archived { get; set; } = null;

        public bool ShouldSerializeArchived() => Archived != null;

        [JsonProperty("deprecated")]
        public bool? Deprecated { get; set; } = null;

        public bool ShouldSerializeDeprecated() => Deprecated != null;

        [JsonProperty("description")]
        public MaybeUnset<string?> Description { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeDescription() => !Description.IsUnset;

        [JsonProperty("featureFlags")]
        public MaybeUnset<List<string>?> FeatureFlags { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeFeatureFlags() => !FeatureFlags.IsUnset;

        [JsonProperty("name")]
        public MaybeUnset<string?> Name { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeName() => !Name.IsUnset;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class StreamEventTypePatch {\n");
            sb.Append("  Archived: ").Append(Archived).Append('\n');
            sb.Append("  Deprecated: ").Append(Deprecated).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
