// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BigQueryConfigPatch
    {
        [JsonProperty("projectId")]
        public string? ProjectId { get; set; } = null;

        public bool ShouldSerializeProjectId() => ProjectId != null;

        [JsonProperty("datasetId")]
        public string? DatasetId { get; set; } = null;

        public bool ShouldSerializeDatasetId() => DatasetId != null;

        [JsonProperty("tableId")]
        public string? TableId { get; set; } = null;

        public bool ShouldSerializeTableId() => TableId != null;

        [JsonProperty("credentials")]
        public string? Credentials { get; set; } = null;

        public bool ShouldSerializeCredentials() => Credentials != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BigQueryConfigPatch {\n");
            sb.Append("  ProjectId: ").Append(ProjectId).Append('\n');
            sb.Append("  DatasetId: ").Append(DatasetId).Append('\n');
            sb.Append("  TableId: ").Append(TableId).Append('\n');
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
