// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration for a SNS sink.
    /// <summary>
    public class SnsConfig
    {
        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        [JsonProperty("endpointUrl")]
        public string? EndpointUrl { get; set; } = null;

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("secretAccessKey", Required = Required.Always)]
        public required string SecretAccessKey { get; set; }

        [JsonProperty("topicArn", Required = Required.Always)]
        public required string TopicArn { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SnsConfig {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  EndpointUrl: ").Append(EndpointUrl).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("  TopicArn: ").Append(TopicArn).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
