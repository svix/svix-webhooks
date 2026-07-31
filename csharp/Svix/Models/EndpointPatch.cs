// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointPatch
    {
        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        public bool ShouldSerializeDescription() => Description != null;

        [JsonProperty("throttleRate")]
        public MaybeUnset<ushort?> ThrottleRate { get; set; } = MaybeUnset<ushort?>.Unset();

        public bool ShouldSerializeThrottleRate() => !ThrottleRate.IsUnset;

        [JsonProperty("uid")]
        public MaybeUnset<string?> Uid { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeUid() => !Uid.IsUnset;

        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        public bool ShouldSerializeUrl() => Url != null;

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        public bool ShouldSerializeDisabled() => Disabled != null;

        [JsonProperty("eventTypes")]
        public MaybeUnset<List<string>?> EventTypes { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeEventTypes() => !EventTypes.IsUnset;

        [JsonProperty("channels")]
        public MaybeUnset<List<string>?> Channels { get; set; } = MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeChannels() => !Channels.IsUnset;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        public bool ShouldSerializeMetadata() => Metadata != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointPatch {\n");
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  ThrottleRate: ").Append(ThrottleRate).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Disabled: ").Append(Disabled).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
