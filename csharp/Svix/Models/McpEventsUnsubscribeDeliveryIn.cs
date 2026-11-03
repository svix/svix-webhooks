// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class McpEventsUnsubscribeDeliveryIn
    {
        [JsonProperty("mode", Required = Required.Always)]
        public required McpEventsDeliveryMode Mode { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class McpEventsUnsubscribeDeliveryIn {\n");
            sb.Append("  Mode: ").Append(Mode).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
