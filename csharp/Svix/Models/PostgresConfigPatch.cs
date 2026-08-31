// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PostgresConfigPatch
    {
        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        public bool ShouldSerializeUrl() => Url != null;

        [JsonProperty("password")]
        public string? Password { get; set; } = null;

        public bool ShouldSerializePassword() => Password != null;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        public bool ShouldSerializeTableName() => TableName != null;

        [JsonProperty("sslRootCert")]
        public string? SslRootCert { get; set; } = null;

        public bool ShouldSerializeSslRootCert() => SslRootCert != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PostgresConfigPatch {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Password: ").Append(Password).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("  SslRootCert: ").Append(SslRootCert).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
