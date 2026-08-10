// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RabbitMqConfigOut
    {
        [JsonProperty("routingKey", Required = Required.Always)]
        public required string RoutingKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RabbitMqConfigOut {\n");
            sb.Append("  RoutingKey: ").Append(RoutingKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
