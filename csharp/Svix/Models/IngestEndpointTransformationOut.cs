// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IngestEndpointTransformationOut
    {
        [JsonProperty("code")]
        public string? Code { get; set; } = null;

        [JsonProperty("enabled")]
        public bool? Enabled { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestEndpointTransformationOut {\n");
            sb.Append("  Code: ").Append(Code).Append('\n');
            sb.Append("  Enabled: ").Append(Enabled).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
