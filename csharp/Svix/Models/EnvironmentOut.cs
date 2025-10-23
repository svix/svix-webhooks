// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EnvironmentOut
    {
        [JsonProperty("connectors", Required = Required.Always)]
        public required List<ConnectorOut> Connectors { get; set; }

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("eventTypes", Required = Required.Always)]
        public required List<EventTypeOut> EventTypes { get; set; }

        [JsonProperty("settings")]
        public Object? Settings { get; set; } = null;

        [JsonProperty("version")]
        public long? Version { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EnvironmentOut {\n");
            sb.Append("  Connectors: ").Append(Connectors).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Settings: ").Append(Settings).Append('\n');
            sb.Append("  Version: ").Append(Version).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
