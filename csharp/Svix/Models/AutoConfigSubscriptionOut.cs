// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AutoConfigSubscriptionOut
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("tokenCensored", Required = Required.Always)]
        public required string TokenCensored { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("endpId")]
        public string? EndpId { get; set; } = null;

        [JsonProperty("destId")]
        public string? DestId { get; set; } = null;

        [JsonProperty("status", Required = Required.Always)]
        public required Status Status { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AutoConfigSubscriptionOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  TokenCensored: ").Append(TokenCensored).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  EndpId: ").Append(EndpId).Append('\n');
            sb.Append("  DestId: ").Append(DestId).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
