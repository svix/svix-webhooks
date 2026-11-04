// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PostgresConfigIn
    {
        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("password")]
        public string? Password { get; set; } = null;

        [JsonProperty("tableName", Required = Required.Always)]
        public required string TableName { get; set; }

        [JsonProperty("sslRootCert")]
        public string? SslRootCert { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PostgresConfigIn {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Password: ").Append(Password).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("  SslRootCert: ").Append(SslRootCert).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
