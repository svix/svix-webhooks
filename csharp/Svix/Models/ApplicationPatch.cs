// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApplicationPatch : BaseModel
    {
        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("name")]
        public string? Name { get; set; } = null;

        [JsonProperty("rateLimit")]
        public MaybeUnset<ushort?> RateLimit { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeRateLimit() => !RateLimit.IsUnset;

        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;
    }
}
