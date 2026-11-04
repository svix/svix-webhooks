// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class McpEventsSubscribeOut
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("endpointId", Required = Required.Always)]
        public required string EndpointId { get; set; }

        [JsonProperty("refreshBefore")]
        public DateTime? RefreshBefore { get; set; } = null;

        [JsonProperty("cursor")]
        public string? Cursor { get; set; } = null;

        [JsonProperty("truncated", Required = Required.Always)]
        public required bool Truncated { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class McpEventsSubscribeOut {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  RefreshBefore: ").Append(RefreshBefore).Append('\n');
            sb.Append("  Cursor: ").Append(Cursor).Append('\n');
            sb.Append("  Truncated: ").Append(Truncated).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
