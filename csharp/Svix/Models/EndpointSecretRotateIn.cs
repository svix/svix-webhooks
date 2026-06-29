// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointSecretRotateIn
    {
        [JsonProperty("gracePeriodSeconds")]
        public uint? GracePeriodSeconds { get; set; } = null;

        [JsonProperty("key")]
        public string? Key { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointSecretRotateIn {\n");
            sb.Append("  GracePeriodSeconds: ").Append(GracePeriodSeconds).Append('\n');
            sb.Append("  Key: ").Append(Key).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
