// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppPortalAccessIn
    {
        [JsonProperty("application")]
        public ApplicationIn? Application { get; set; } = null;

        [JsonProperty("capabilities")]
        public List<AppPortalCapability>? Capabilities { get; set; } = null;

        [JsonProperty("expiry")]
        public ulong? Expiry { get; set; } = null;

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("readOnly")]
        public bool? ReadOnly { get; set; } = null;

        [JsonProperty("sessionId")]
        public string? SessionId { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AppPortalAccessIn {\n");
            sb.Append("  Application: ").Append(Application).Append('\n');
            sb.Append("  Capabilities: ").Append(Capabilities).Append('\n');
            sb.Append("  Expiry: ").Append(Expiry).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  ReadOnly: ").Append(ReadOnly).Append('\n');
            sb.Append("  SessionId: ").Append(SessionId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
