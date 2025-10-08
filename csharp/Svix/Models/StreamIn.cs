// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class StreamIn
    {
        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class StreamIn {\n");
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
