// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventIn
    {
        [JsonProperty("eventType", Required = Required.Always)]
        public required string EventType { get; set; }

        [JsonProperty("payload", Required = Required.Always)]
        public required string Payload { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventIn {\n");
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  Payload: ").Append(Payload).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
