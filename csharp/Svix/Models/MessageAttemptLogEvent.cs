// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent after message attempts are made. Contains metadata about message attempts and their results. In order to reduce the frequency of webhooks, these are sent in batches periodically.
    /// <summary>
    public class MessageAttemptLogEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public required List<MessageAttemptLog> Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public required string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptLogEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
