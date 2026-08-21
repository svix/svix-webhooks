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

        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SnsConfigOut {\n");
            sb.Append("  TopicArn: ").Append(TopicArn).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
