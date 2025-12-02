// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageAttemptLog
    {
        [JsonProperty("appId", Required = Required.Always)]
        public required string AppId { get; set; }

        [JsonProperty("appUid")]
        public string? AppUid { get; set; } = null;

        [JsonProperty("attemptCount", Required = Required.Always)]
        public required ushort AttemptCount { get; set; }

        [JsonProperty("attemptEnd", Required = Required.Always)]
        public required DateTime AttemptEnd { get; set; }

        [JsonProperty("attemptId", Required = Required.Always)]
        public required string AttemptId { get; set; }

        [JsonProperty("attemptStart", Required = Required.Always)]
        public required DateTime AttemptStart { get; set; }

        [JsonProperty("endpointId", Required = Required.Always)]
        public required string EndpointId { get; set; }

        [JsonProperty("eventType")]
        public string? EventType { get; set; } = null;

        [JsonProperty("httpTimes")]
        public HttpAttemptTimes? HttpTimes { get; set; } = null;

        [JsonProperty("msgCreated", Required = Required.Always)]
        public required DateTime MsgCreated { get; set; }

        [JsonProperty("msgEventId")]
        public string? MsgEventId { get; set; } = null;

        [JsonProperty("msgId", Required = Required.Always)]
        public required string MsgId { get; set; }

        [JsonProperty("orgId", Required = Required.Always)]
        public required string OrgId { get; set; }

        [JsonProperty("responseStatusCode", Required = Required.Always)]
        public required short ResponseStatusCode { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required MessageStatus Status { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptLog {\n");
            sb.Append("  AppId: ").Append(AppId).Append('\n');
            sb.Append("  AppUid: ").Append(AppUid).Append('\n');
            sb.Append("  AttemptCount: ").Append(AttemptCount).Append('\n');
            sb.Append("  AttemptEnd: ").Append(AttemptEnd).Append('\n');
            sb.Append("  AttemptId: ").Append(AttemptId).Append('\n');
            sb.Append("  AttemptStart: ").Append(AttemptStart).Append('\n');
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  HttpTimes: ").Append(HttpTimes).Append('\n');
            sb.Append("  MsgCreated: ").Append(MsgCreated).Append('\n');
            sb.Append("  MsgEventId: ").Append(MsgEventId).Append('\n');
            sb.Append("  MsgId: ").Append(MsgId).Append('\n');
            sb.Append("  OrgId: ").Append(OrgId).Append('\n');
            sb.Append("  ResponseStatusCode: ").Append(ResponseStatusCode).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
