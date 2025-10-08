// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GoogleCloudStoragePatchConfig
    {
        [JsonProperty("bucket")]
        public string? Bucket { get; set; } = null;

        [JsonProperty("credentials")]
        public string? Credentials { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GoogleCloudStoragePatchConfig {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
