// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SinkTransformationOut
    {
        [JsonProperty("code")]
        public string? Code { get; set; } = null;

        [JsonProperty("enabled", Required = Required.Always)]
        public required bool Enabled { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SinkTransformationOut {\n");
            sb.Append("  Code: ").Append(Code).Append('\n');
            sb.Append("  Enabled: ").Append(Enabled).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
