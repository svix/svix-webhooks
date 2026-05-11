// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SubscribeIn
    {
        [JsonProperty("endpoint", Required = Required.Always)]
        public required EndpointIn Endpoint { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SubscribeIn {\n");
            sb.Append("  Endpoint: ").Append(Endpoint).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
