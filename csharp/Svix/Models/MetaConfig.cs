// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MetaConfig
    {
        [JsonProperty("secret", Required = Required.Always)]
        public required string Secret { get; set; }

        [JsonProperty("verifyToken", Required = Required.Always)]
        public required string VerifyToken { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MetaConfig {\n");
            sb.Append("  Secret: ").Append(Secret).Append('\n');
            sb.Append("  VerifyToken: ").Append(VerifyToken).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
