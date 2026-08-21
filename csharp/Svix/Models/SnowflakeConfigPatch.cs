// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SnowflakeConfigPatch
    {
        [JsonProperty("privateKey")]
        public string? PrivateKey { get; set; } = null;

        public bool ShouldSerializePrivateKey() => PrivateKey != null;

        [JsonProperty("accountIdentifier")]
        public string? AccountIdentifier { get; set; } = null;

        public bool ShouldSerializeAccountIdentifier() => AccountIdentifier != null;

        [JsonProperty("userId")]
        public string? UserId { get; set; } = null;

        public bool ShouldSerializeUserId() => UserId != null;

        [JsonProperty("dbName")]
        public string? DbName { get; set; } = null;

        public bool ShouldSerializeDbName() => DbName != null;

        [JsonProperty("schemaName")]
        public string? SchemaName { get; set; } = null;

        public bool ShouldSerializeSchemaName() => SchemaName != null;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        public bool ShouldSerializeTableName() => TableName != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SnowflakeConfigPatch {\n");
            sb.Append("  PrivateKey: ").Append(PrivateKey).Append('\n');
            sb.Append("  AccountIdentifier: ").Append(AccountIdentifier).Append('\n');
            sb.Append("  UserId: ").Append(UserId).Append('\n');
            sb.Append("  DbName: ").Append(DbName).Append('\n');
            sb.Append("  SchemaName: ").Append(SchemaName).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
