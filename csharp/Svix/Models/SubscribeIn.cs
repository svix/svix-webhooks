// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SubscribeIn
    {
        [JsonProperty("endpoint")]
        public EndpointIn? Endpoint { get; set; } = null;

        [JsonProperty("sink")]
        public AutoConfigSinkType? Sink { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SubscribeIn {\n");
            sb.Append("  Endpoint: ").Append(Endpoint).Append('\n');
            sb.Append("  Sink: ").Append(Sink).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
