// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GoogleCloudStorageConfigPatch
    {
        [JsonProperty("bucket")]
        public string? Bucket { get; set; } = null;

        public bool ShouldSerializeBucket() => Bucket != null;

        [JsonProperty("credentials")]
        public string? Credentials { get; set; } = null;

        public bool ShouldSerializeCredentials() => Credentials != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GoogleCloudStorageConfigPatch {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
