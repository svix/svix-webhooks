// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RotateSubscriptionIn2
    {
        [JsonProperty("signingSecret")]
        public EndpointSecretRotateIn? SigningSecret { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RotateSubscriptionIn2 {\n");
            sb.Append("  SigningSecret: ").Append(SigningSecret).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
