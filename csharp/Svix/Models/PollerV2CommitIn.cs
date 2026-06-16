// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PollerV2CommitIn
    {
        [JsonProperty("offset", Required = Required.Always)]
        public required ulong Offset { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PollerV2CommitIn {\n");
            sb.Append("  Offset: ").Append(Offset).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
