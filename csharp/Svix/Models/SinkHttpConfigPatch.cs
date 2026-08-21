// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SinkHttpConfigPatch
    {
        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        public bool ShouldSerializeUrl() => Url != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SinkHttpConfigPatch {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
