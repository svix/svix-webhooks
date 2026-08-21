// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RabbitMqConfigPatch
    {
        [JsonProperty("routingKey")]
        public string? RoutingKey { get; set; } = null;

        public bool ShouldSerializeRoutingKey() => RoutingKey != null;

        [JsonProperty("uri")]
        public string? Uri { get; set; } = null;

        public bool ShouldSerializeUri() => Uri != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RabbitMqConfigPatch {\n");
            sb.Append("  RoutingKey: ").Append(RoutingKey).Append('\n');
            sb.Append("  Uri: ").Append(Uri).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
