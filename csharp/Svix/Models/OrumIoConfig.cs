// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class OrumIoConfig
    {
        [JsonProperty("publicKey", Required = Required.Always)]
        public required string PublicKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class OrumIoConfig {\n");
            sb.Append("  PublicKey: ").Append(PublicKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
