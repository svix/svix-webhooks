// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageEndpointOut
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required MessageStatus Status { get; set; }

        [JsonProperty("statusText", Required = Required.Always)]
        public required MessageStatusText StatusText { get; set; }

        [JsonProperty("nextAttempt")]
        public DateTime? NextAttempt { get; set; } = null;

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public required string Description { get; set; }

        [JsonProperty("throttleRate")]
        public ushort? ThrottleRate { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("eventTypes")]
        public List<string>? EventTypes { get; set; } = null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageEndpointOut {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  StatusText: ").Append(StatusText).Append('\n');
            sb.Append("  NextAttempt: ").Append(NextAttempt).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  ThrottleRate: ").Append(ThrottleRate).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Disabled: ").Append(Disabled).Append('\n');
            sb.Append("  EventTypes: ").Append(EventTypes).Append('\n');
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
