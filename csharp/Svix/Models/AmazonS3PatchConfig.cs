// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AmazonS3PatchConfig
    {
        [JsonProperty("accessKeyId")]
        public string? AccessKeyId { get; set; } = null;

        [JsonProperty("bucket")]
        public string? Bucket { get; set; } = null;

        [JsonProperty("region")]
        public string? Region { get; set; } = null;

        [JsonProperty("secretAccessKey")]
        public string? SecretAccessKey { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AmazonS3PatchConfig {\n");
            sb.Append("  AccessKeyId: ").Append(AccessKeyId).Append('\n');
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  Region: ").Append(Region).Append('\n');
            sb.Append("  SecretAccessKey: ").Append(SecretAccessKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
