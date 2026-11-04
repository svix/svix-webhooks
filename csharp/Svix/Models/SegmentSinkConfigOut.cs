// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SegmentSinkConfigOut
    {
        [JsonProperty("host", Required = Required.Always)]
        public required string Host { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SegmentSinkConfigOut {\n");
            sb.Append("  Host: ").Append(Host).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
