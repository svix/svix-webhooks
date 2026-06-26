// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventBridgeConfig
    {
        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        [JsonProperty("detailType")]
        public string? DetailType { get; set; } = null;

        [JsonProperty("eventBusName", Required = Required.Always)]
        public required string EventBusName { get; set; }

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("secretAccessKey", Required = Required.Always)]
        public required string SecretAccessKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventBridgeConfig {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  DetailType: ").Append(DetailType).Append('\n');
            sb.Append("  EventBusName: ").Append(EventBusName).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
