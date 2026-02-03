// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BulkReplayIn
    {
        [JsonProperty("channel")]
        public string? Channel { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("since", Required = Required.Always)]
        public required DateTime Since { get; set; }

        [JsonProperty("status")]
        public MessageStatus? Status { get; set; } = null;

        [JsonProperty("tag")]
        public string? Tag { get; set; } = null;

        [JsonProperty("until")]
        public DateTime? Until { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BulkReplayIn {\n");
            sb.Append("  Channel: ").Append(Channel).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Since: ").Append(Since).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Tag: ").Append(Tag).Append('\n');
            sb.Append("  Until: ").Append(Until).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
