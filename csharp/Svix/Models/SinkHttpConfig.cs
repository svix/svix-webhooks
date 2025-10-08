// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SinkHttpConfig
    {
        [JsonProperty("headers")]
        public Dictionary<string, string>? Headers { get; set; } = null;

        [JsonProperty("key")]
        public string? Key { get; set; } = null;

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SinkHttpConfig {\n");
            sb.Append("  Headers: ").Append(Headers).Append('\n');
            sb.Append("  Key: ").Append(Key).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
