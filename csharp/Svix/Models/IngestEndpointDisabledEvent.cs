// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call.
    /// <summary>
    public class IngestEndpointDisabledEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public required IngestEndpointDisabledEventData Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestEndpointDisabledEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
