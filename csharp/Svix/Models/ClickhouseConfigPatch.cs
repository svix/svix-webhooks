// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ClickhouseConfigPatch
    {
        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        public bool ShouldSerializeUrl() => Url != null;

        [JsonProperty("username")]
        public string? Username { get; set; } = null;

        public bool ShouldSerializeUsername() => Username != null;

        [JsonProperty("password")]
        public string? Password { get; set; } = null;

        public bool ShouldSerializePassword() => Password != null;

        [JsonProperty("database")]
        public string? Database { get; set; } = null;

        public bool ShouldSerializeDatabase() => Database != null;

        [JsonProperty("tableName")]
        public string? TableName { get; set; } = null;

        public bool ShouldSerializeTableName() => TableName != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ClickhouseConfigPatch {\n");
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
