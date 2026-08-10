// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RedshiftConfigOut
    {
        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("clusterIdentifier")]
        public string? ClusterIdentifier { get; set; } = null;

        [JsonProperty("dbUser")]
        public string? DbUser { get; set; } = null;

        [JsonProperty("workgroupName")]
        public string? WorkgroupName { get; set; } = null;

        [JsonProperty("dbName")]
        public string? DbName { get; set; } = null;

        [JsonProperty("schemaName")]
        public string? SchemaName { get; set; } = null;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RedshiftConfigOut {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  ClusterIdentifier: ").Append(ClusterIdentifier).Append('\n');
            sb.Append("  DbUser: ").Append(DbUser).Append('\n');
            sb.Append("  WorkgroupName: ").Append(WorkgroupName).Append('\n');
            sb.Append("  DbName: ").Append(DbName).Append('\n');
            sb.Append("  SchemaName: ").Append(SchemaName).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
