// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GoogleCloudStorageConfigOut
    {
        [JsonProperty("bucket", Required = Required.Always)]
        public required string Bucket { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GoogleCloudStorageConfigOut {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
