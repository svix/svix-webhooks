// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BigQueryPatchConfig
    {
        [JsonProperty("credentials")]
        public string? Credentials { get; set; } = null;

        [JsonProperty("datasetId")]
        public string? DatasetId { get; set; } = null;

        [JsonProperty("projectId")]
        public string? ProjectId { get; set; } = null;

        [JsonProperty("tableId")]
        public string? TableId { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BigQueryPatchConfig {\n");
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("  DatasetId: ").Append(DatasetId).Append('\n');
            sb.Append("  ProjectId: ").Append(ProjectId).Append('\n');
            sb.Append("  TableId: ").Append(TableId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
