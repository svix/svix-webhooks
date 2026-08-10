// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SinkOtelTracingConfigOut
    {
        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("headers", Required = Required.Always)]
        public required EndpointHeadersOut Headers { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SinkOtelTracingConfigOut {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Headers: ").Append(Headers).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
