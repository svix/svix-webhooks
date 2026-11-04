// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AutoConfigOut
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("token", Required = Required.Always)]
        public required string Token { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AutoConfigOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  Token: ").Append(Token).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
