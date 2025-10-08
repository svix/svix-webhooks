// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventOut
    {
        [JsonProperty("eventType", Required = Required.Always)]
        public required string EventType { get; set; }

        [JsonProperty("payload", Required = Required.Always)]
        public required string Payload { get; set; }

        [JsonProperty("timestamp", Required = Required.Always)]
        public required DateTime Timestamp { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventOut {\n");
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  Payload: ").Append(Payload).Append('\n');
            sb.Append("  Timestamp: ").Append(Timestamp).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
