// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call.
    /// <summary>
    public class IngestEndpointDisabledEventData
    {
        [JsonProperty("endpointId", Required = Required.Always)]
        public required string EndpointId { get; set; }

        [JsonProperty("endpointUid")]
        public string? EndpointUid { get; set; } = null;

        [JsonProperty("failSince")]
        public DateTime? FailSince { get; set; } = null;

        [JsonProperty("sourceId", Required = Required.Always)]
        public required string SourceId { get; set; }

        [JsonProperty("trigger")]
        public EndpointDisabledTrigger? Trigger { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestEndpointDisabledEventData {\n");
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  EndpointUid: ").Append(EndpointUid).Append('\n');
            sb.Append("  FailSince: ").Append(FailSince).Append('\n');
            sb.Append("  SourceId: ").Append(SourceId).Append('\n');
            sb.Append("  Trigger: ").Append(Trigger).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
