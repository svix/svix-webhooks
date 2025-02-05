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
        public string? Description { get; set; } = null;

        public bool ShouldSerializeDescription() => Description != null;

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        public bool ShouldSerializeDisabled() => Disabled != null;

        [JsonProperty("filterTypes")]
        public MaybeUnset<List<string>?> FilterTypes { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeFilterTypes() => !FilterTypes.IsUnset;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

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
        public string? Url { get; set; } = null;

        public bool ShouldSerializeUrl() => Url != null;

        [JsonProperty("version")]
        public ushort? Version { get; set; } = null;

        public bool ShouldSerializeVersion() => Version != null;
    }
}
