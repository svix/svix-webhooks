// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class McpEventsSubscribeIn
    {
        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("arguments")]
        public Object? Arguments { get; set; } = null;

        [JsonProperty("delivery", Required = Required.Always)]
        public required McpEventsSubscribeDeliveryIn Delivery { get; set; }

        [JsonProperty("ttlMs")]
        public ulong? TtlMs { get; set; } = null;

        [JsonProperty("secretGracePeriodSeconds")]
        public uint? SecretGracePeriodSeconds { get; set; } = null;

        [JsonProperty("challengeMode")]
        public EndpointChallengeMode? ChallengeMode { get; set; } = null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class McpEventsSubscribeIn {\n");
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Arguments: ").Append(Arguments).Append('\n');
            sb.Append("  Delivery: ").Append(Delivery).Append('\n');
            sb.Append("  TtlMs: ").Append(TtlMs).Append('\n');
            sb.Append("  SecretGracePeriodSeconds: ").Append(SecretGracePeriodSeconds).Append('\n');
            sb.Append("  ChallengeMode: ").Append(ChallengeMode).Append('\n');
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
