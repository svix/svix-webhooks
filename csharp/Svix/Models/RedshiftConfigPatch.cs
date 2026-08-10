// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RedshiftConfigPatch
    {
        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        public bool ShouldSerializeAccessKeyId() => AccessKeyId != null;

        [JsonProperty("secretAccessKey")]
        public string? SecretAccessKey { get; set; } = null;

        public bool ShouldSerializeSecretAccessKey() => SecretAccessKey != null;

        [JsonProperty("region")]
        public string? Region { get; set; } = null;

        public bool ShouldSerializeRegion() => Region != null;

        [JsonProperty("dbName")]
        public string? DbName { get; set; } = null;

        public bool ShouldSerializeDbName() => DbName != null;

        [JsonProperty("schemaName")]
        public MaybeUnset<string?> SchemaName { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeSchemaName() => !SchemaName.IsUnset;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        public bool ShouldSerializeTableName() => TableName != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RedshiftConfigPatch {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  DbName: ").Append(DbName).Append('\n');
            sb.Append("  SchemaName: ").Append(SchemaName).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
