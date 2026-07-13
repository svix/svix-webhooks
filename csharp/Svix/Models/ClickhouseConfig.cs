// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ClickhouseConfig
    {
        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("username", Required = Required.Always)]
        public required string Username { get; set; }

        [JsonProperty("password", Required = Required.Always)]
        public required string Password { get; set; }

        [JsonProperty("database")]
        public string? Database { get; set; } = null;

        [JsonProperty("tableName", Required = Required.Always)]
        public required string TableName { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ClickhouseConfig {\n");
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Username: ").Append(Username).Append('\n');
            sb.Append("  Password: ").Append(Password).Append('\n');
            sb.Append("  Database: ").Append(Database).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
