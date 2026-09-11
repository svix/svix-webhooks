// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SnsConfigOut
    {
        [JsonProperty("topicArn", Required = Required.Always)]
        public required string TopicArn { get; set; }

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        [JsonProperty("roleArn")]
        public string? RoleArn { get; set; } = null;

        [JsonProperty("externalId")]
        public string? ExternalId { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SnsConfigOut {\n");
            sb.Append("  TopicArn: ").Append(TopicArn).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  RoleArn: ").Append(RoleArn).Append('\n');
            sb.Append("  ExternalId: ").Append(ExternalId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
