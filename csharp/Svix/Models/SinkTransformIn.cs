// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SinkTransformIn
    {
        [JsonProperty("code")]
        public string? Code { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SinkTransformIn {\n");
            sb.Append("  Code: ").Append(Code).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
