// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApplicationPatch : BaseModel
    {
        [JsonProperty("metadata")]
        public MaybeUnset<Dictionary<string, string>?> Metadata { get; set; } =
            MaybeUnset<Dictionary<string, string>?>.Unset();

        public bool ShouldSerializeMetadata() => !Metadata.IsUnset;

        [JsonProperty("name")]
        public MaybeUnset<string?> Name { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeName() => !Name.IsUnset;

        [JsonProperty("rateLimit")]
        public MaybeUnset<ushort?> RateLimit { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeRateLimit() => !RateLimit.IsUnset;

        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;
    }
}
