// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class StreamPortalAccessIn
    {
        [JsonProperty("expiry")]
        public ulong? Expiry { get; set; } = null;

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("sessionId")]
        public string? SessionId { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class StreamPortalAccessIn {\n");
            sb.Append("  Expiry: ").Append(Expiry).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  SessionId: ").Append(SessionId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
