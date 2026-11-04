// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class S3ConfigOut
    {
        [JsonProperty("bucket", Required = Required.Always)]
        public required string Bucket { get; set; }

        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("endpointUrl")]
        public string? EndpointUrl { get; set; } = null;

        [JsonProperty("roleArn")]
        public string? RoleArn { get; set; } = null;

        [JsonProperty("externalId")]
        public string? ExternalId { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class S3ConfigOut {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  EndpointUrl: ").Append(EndpointUrl).Append('\n');
            sb.Append("  RoleArn: ").Append(RoleArn).Append('\n');
            sb.Append("  ExternalId: ").Append(ExternalId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
