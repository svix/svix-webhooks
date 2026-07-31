// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointHeadersPatchIn
    {
        [JsonProperty("headers", Required = Required.Always)]
        public required Dictionary<string, string> Headers { get; set; }

        [JsonProperty("deleteHeaders")]
        public List<string>? DeleteHeaders { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointHeadersPatchIn {\n");
            sb.Append("  Headers: ").Append(Headers).Append('\n');
            sb.Append("  DeleteHeaders: ").Append(DeleteHeaders).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
