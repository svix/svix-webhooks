// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class HttpPatchConfig
    {
        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class HttpPatchConfig {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
