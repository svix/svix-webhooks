// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointOauthOut
    {
        [JsonProperty("enabled", Required = Required.Always)]
        public required bool Enabled { get; set; }

        [JsonProperty("config")]
        public EndpointOauthConfigOut? Config { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointOauthOut {\n");
            sb.Append("  Enabled: ").Append(Enabled).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
