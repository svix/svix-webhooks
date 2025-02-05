// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointPatch : BaseModel
    {
        [JsonProperty("channels")]
        public MaybeUnset<List<string>?> Channels { get; set; } = MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeChannels() => !Channels.IsUnset;

        [JsonProperty("description")]
        public MaybeUnset<string?> Description { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeDescription() => !Description.IsUnset;

        [JsonProperty("disabled")]
        public MaybeUnset<bool?> Disabled { get; set; } = MaybeUnset<bool?>.Unset();

        public bool ShouldSerializeDisabled() => !Disabled.IsUnset;

        [JsonProperty("filterTypes")]
        public MaybeUnset<List<string>?> FilterTypes { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeFilterTypes() => !FilterTypes.IsUnset;

        [JsonProperty("metadata")]
        public MaybeUnset<Dictionary<string, string>?> Metadata { get; set; } =
            MaybeUnset<Dictionary<string, string>?>.Unset();

        public bool ShouldSerializeMetadata() => !Metadata.IsUnset;

        [JsonProperty("rateLimit")]
        public MaybeUnset<ushort?> RateLimit { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeRateLimit() => !RateLimit.IsUnset;

        [JsonProperty("secret")]
        public MaybeUnset<string?> Secret { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeSecret() => !Secret.IsUnset;

        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        [JsonProperty("url")]
        public MaybeUnset<string?> Url { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUrl() => !Url.IsUnset;

        [JsonProperty("version")]
        public MaybeUnset<ushort?> Version { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeVersion() => !Version.IsUnset;
    }
}
