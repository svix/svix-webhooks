// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ClickhousePatchConfig
    {
        [JsonProperty("database")]
        public string? Database { get; set; } = null;

        [JsonProperty("password")]
        public string? Password { get; set; } = null;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        [JsonProperty("username")]
        public string? Username { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ClickhousePatchConfig {\n");
            sb.Append("  Database: ").Append(Database).Append('\n');
            sb.Append("  Password: ").Append(Password).Append('\n');
            sb.Append("  TableName: ").Append(TableName).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Username: ").Append(Username).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
