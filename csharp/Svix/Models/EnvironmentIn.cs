// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EnvironmentIn
    {
        [JsonProperty("connectors")]
        public List<ConnectorIn>? Connectors { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<EventTypeIn>? EventTypes { get; set; } = null;

        [JsonProperty("settings")]
        public Object? Settings { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EnvironmentIn {\n");
            sb.Append("  Connectors: ").Append(Connectors).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Settings: ").Append(Settings).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
