// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class S3Config
    {
        [JsonProperty("accessKeyId", Required = Required.Always)]
        public required string AccessKeyId { get; set; }

        [JsonProperty("bucket", Required = Required.Always)]
        public required string Bucket { get; set; }

        [JsonProperty("region", Required = Required.Always)]
        public required string Region { get; set; }

        [JsonProperty("secretAccessKey", Required = Required.Always)]
        public required string SecretAccessKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class S3Config {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
