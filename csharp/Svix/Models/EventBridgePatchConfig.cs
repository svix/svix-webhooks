// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventBridgePatchConfig
    {
        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        [JsonProperty("detailType")]
        public string? DetailType { get; set; } = null;

        [JsonProperty("eventBusName")]
        public string? EventBusName { get; set; } = null;

        [JsonProperty("region")]
        public string? Region { get; set; } = null;

        [JsonProperty("secretAccessKey")]
        public string? SecretAccessKey { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventBridgePatchConfig {\n");
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
