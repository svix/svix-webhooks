// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApplicationPatch
    {
        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

        [JsonProperty("name")]
        public string? Name { get; set; } = null;

        public bool ShouldSerializeName() => Name != null;

        [JsonProperty("rateLimit")]
        public MaybeUnset<ushort?> RateLimit { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeRateLimit() => !RateLimit.IsUnset;

        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ApplicationPatch {\n");
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  RateLimit: ").Append(RateLimit).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
