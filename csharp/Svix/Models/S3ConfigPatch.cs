// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class S3ConfigPatch
    {
        [JsonProperty("bucket")]
        public string? Bucket { get; set; } = null;

        public bool ShouldSerializeBucket() => Bucket != null;

        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        public bool ShouldSerializeAccessKeyId() => AccessKeyId != null;

        [JsonProperty("secretAccessKey")]
        public string? SecretAccessKey { get; set; } = null;

        public bool ShouldSerializeSecretAccessKey() => SecretAccessKey != null;

        [JsonProperty("roleArn")]
        public string? RoleArn { get; set; } = null;

        public bool ShouldSerializeRoleArn() => RoleArn != null;

        [JsonProperty("externalId")]
        public string? ExternalId { get; set; } = null;

        public bool ShouldSerializeExternalId() => ExternalId != null;

        [JsonProperty("region")]
        public string? Region { get; set; } = null;

        public bool ShouldSerializeRegion() => Region != null;

        [JsonProperty("endpointUrl")]
        public string? EndpointUrl { get; set; } = null;

        public bool ShouldSerializeEndpointUrl() => EndpointUrl != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class S3ConfigPatch {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("  RoleArn: ").Append(RoleArn).Append('\n');
            sb.Append("  ExternalId: ").Append(ExternalId).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  EndpointUrl: ").Append(EndpointUrl).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
