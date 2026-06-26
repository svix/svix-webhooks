// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SqsPatchConfig
    {
        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        [JsonProperty("endpointUrl")]
        public string? EndpointUrl { get; set; } = null;

        [JsonProperty("queueUrl")]
        public string? QueueUrl { get; set; } = null;

        [JsonProperty("region")]
        public string? Region { get; set; } = null;

        [JsonProperty("secretAccessKey")]
        public string? SecretAccessKey { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SqsPatchConfig {\n");
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
