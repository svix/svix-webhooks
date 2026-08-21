// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BulkExpungeContentsIn
    {
        [JsonProperty("ids")]
        public List<string>? Ids { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BulkExpungeContentsIn {\n");
            sb.Append("  Ids: ").Append(Ids).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
