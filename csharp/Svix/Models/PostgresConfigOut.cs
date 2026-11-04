// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PostgresConfigOut
    {
        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("tableName", Required = Required.Always)]
        public required string TableName { get; set; }

        [JsonProperty("sslRootCert")]
        public string? SslRootCert { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PostgresConfigOut {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("  SslRootCert: ").Append(SslRootCert).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
