// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "ingest.message.attempt.exhausted" type, after it's failed four times as a "ingest.message.attempt.failing" event, or after it's recovered as a "ingest.message.attempt.recovered" event.
    /// <summary>
    public class IngestMessageAttemptRecoveredEventData
    {
        [JsonProperty("endpointId", Required = Required.Always)]
        public required string EndpointId { get; set; }

        [JsonProperty("lastAttempt", Required = Required.Always)]
        public required MessageAttemptFailedData LastAttempt { get; set; }

        [JsonProperty("msgEventId")]
        public string? MsgEventId { get; set; } = null;

        [JsonProperty("msgId", Required = Required.Always)]
        public required string MsgId { get; set; }

        [JsonProperty("sourceId", Required = Required.Always)]
        public required string SourceId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestMessageAttemptRecoveredEventData {\n");
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  LastAttempt: ").Append(LastAttempt).Append('\n');
            sb.Append("  MsgEventId: ").Append(MsgEventId).Append('\n');
            sb.Append("  MsgId: ").Append(MsgId).Append('\n');
            sb.Append("  SourceId: ").Append(SourceId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
