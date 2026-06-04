// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration for an SQS sink.
    /// <summary>
    public class SqsConfig
    {
        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        [JsonProperty("endpointUrl")]
        public string? EndpointUrl { get; set; } = null;

        [JsonProperty("queueUrl", Required = Required.Always)]
        public required string QueueUrl { get; set; }

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("secretAccessKey", Required = Required.Always)]
        public required string SecretAccessKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SqsConfig {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  EndpointUrl: ").Append(EndpointUrl).Append('\n');
            sb.Append("  QueueUrl: ").Append(QueueUrl).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
