// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SqsConfigPatch
    {
        [JsonProperty("queueUrl")]
        public string? QueueUrl { get; set; } = null;

        public bool ShouldSerializeQueueUrl() => QueueUrl != null;

        [JsonProperty("region")]
        public string? Region { get; set; } = null;

        public bool ShouldSerializeRegion() => Region != null;

        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        public bool ShouldSerializeAccessKeyId() => AccessKeyId != null;

        [JsonProperty("secretAccessKey")]
        public string? SecretAccessKey { get; set; } = null;

        public bool ShouldSerializeSecretAccessKey() => SecretAccessKey != null;

        [JsonProperty("endpointUrl")]
        public MaybeUnset<string?> EndpointUrl { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeEndpointUrl() => !EndpointUrl.IsUnset;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SqsConfigPatch {\n");
            sb.Append("  QueueUrl: ").Append(QueueUrl).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("  EndpointUrl: ").Append(EndpointUrl).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
