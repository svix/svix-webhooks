// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventBridgeConfigOut
    {
        [JsonProperty("eventBusName", Required = Required.Always)]
        public required string EventBusName { get; set; }

        [JsonProperty("detailType", Required = Required.Always)]
        public required string DetailType { get; set; }

        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventBridgeConfigOut {\n");
            sb.Append("  EventBusName: ").Append(EventBusName).Append('\n');
            sb.Append("  DetailType: ").Append(DetailType).Append('\n');
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
