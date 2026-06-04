// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration for a Google Cloud BigQuery sink.
    /// <summary>
    public class BigQueryConfig
    {
        [JsonProperty("credentials", Required = Required.Always)]
        public required string Credentials { get; set; }

        [JsonProperty("datasetId", Required = Required.Always)]
        public required string DatasetId { get; set; }

        [JsonProperty("projectId", Required = Required.Always)]
        public required string ProjectId { get; set; }

        [JsonProperty("tableId", Required = Required.Always)]
        public required string TableId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BigQueryConfig {\n");
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("  DatasetId: ").Append(DatasetId).Append('\n');
            sb.Append("  ProjectId: ").Append(ProjectId).Append('\n');
            sb.Append("  TableId: ").Append(TableId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
