// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointMtlsOut
    {
        [JsonProperty("enabled", Required = Required.Always)]
        public required bool Enabled { get; set; }

        [JsonProperty("config")]
        public EndpointMtlsConfigOut? Config { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointMtlsOut {\n");
            sb.Append("  Enabled: ").Append(Enabled).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
