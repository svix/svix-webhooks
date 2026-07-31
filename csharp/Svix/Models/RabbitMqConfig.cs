// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration for a RabbitMq sink.
    /// <summary>
    public class RabbitMqConfig
    {
        [JsonProperty("uri", Required = Required.Always)]
        public required string Uri { get; set; }

        [JsonProperty("routingKey", Required = Required.Always)]
        public required string RoutingKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RabbitMqConfig {\n");
            sb.Append("  Uri: ").Append(Uri).Append('\n');
            sb.Append("  RoutingKey: ").Append(RoutingKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
