// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IngestSourceConsumerPortalAccessIn
    {
        [JsonProperty("expiry")]
        public ulong? Expiry { get; set; } = null;

        [JsonProperty("readOnly")]
        public bool? ReadOnly { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestSourceConsumerPortalAccessIn {\n");
            sb.Append("  Expiry: ").Append(Expiry).Append('\n');
            sb.Append("  ReadOnly: ").Append(ReadOnly).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
