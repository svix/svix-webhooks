// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointMtlsConfigIn
    {
        [JsonProperty("identity", Required = Required.Always)]
        public required string Identity { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointMtlsConfigIn {\n");
            sb.Append("  Identity: ").Append(Identity).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
