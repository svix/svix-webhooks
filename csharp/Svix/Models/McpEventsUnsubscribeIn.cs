// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class McpEventsUnsubscribeIn
    {
        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("arguments")]
        public Object? Arguments { get; set; } = null;

        [JsonProperty("delivery", Required = Required.Always)]
        public required McpEventsUnsubscribeDeliveryIn Delivery { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class McpEventsUnsubscribeIn {\n");
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Arguments: ").Append(Arguments).Append('\n');
            sb.Append("  Delivery: ").Append(Delivery).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
