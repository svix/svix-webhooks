// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BulkExpungeContentsOut
    {
        [JsonProperty("results", Required = Required.Always)]
        public required Dictionary<string, BulkExpungeStatus> Results { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BulkExpungeContentsOut {\n");
            sb.Append("  Results: ").Append(Results).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
