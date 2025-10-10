// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class CreateStreamEventsIn
    {
        [JsonProperty("events", Required = Required.Always)]
        public required List<EventIn> Events { get; set; }

        [JsonProperty("stream")]
        public StreamIn? Stream { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class CreateStreamEventsIn {\n");
            sb.Append("  Events: ").Append(Events).Append('\n');
            sb.Append("  Stream: ").Append(Stream).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
