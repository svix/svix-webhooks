// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessagePrecheckIn
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventType", Required = Required.Always)]
        public required string EventType { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessagePrecheckIn {\n");
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
