// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RabbitMqPatchConfig
    {
        [JsonProperty("routingKey")]
        public string? RoutingKey { get; set; } = null;

        [JsonProperty("uri")]
        public string? Uri { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RabbitMqPatchConfig {\n");
            sb.Append("  RoutingKey: ").Append(RoutingKey).Append('\n');
            sb.Append("  Uri: ").Append(Uri).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
