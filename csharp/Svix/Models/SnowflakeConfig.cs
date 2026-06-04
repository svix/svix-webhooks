// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration parameters for defining a Snowflake sink.
    /// <summary>
    public class SnowflakeConfig
    {
        [JsonProperty("accountIdentifier", Required = Required.Always)]
        public required string AccountIdentifier { get; set; }

        [JsonProperty("dbName")]
        public string? DbName { get; set; } = null;

        [JsonProperty("privateKey", Required = Required.Always)]
        public required string PrivateKey { get; set; }

        [JsonProperty("schemaName")]
        public string? SchemaName { get; set; } = null;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        [JsonProperty("userId", Required = Required.Always)]
        public required string UserId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SnowflakeConfig {\n");
            sb.Append("  AccountIdentifier: ").Append(AccountIdentifier).Append('\n');
            sb.Append("  DbName: ").Append(DbName).Append('\n');
            sb.Append("  PrivateKey: ").Append(PrivateKey).Append('\n');
            sb.Append("  SchemaName: ").Append(SchemaName).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("  UserId: ").Append(UserId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
