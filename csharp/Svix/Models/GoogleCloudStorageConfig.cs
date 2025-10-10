// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration for a Google Cloud Storage sink.
    ///
    /// Write stream events into the named bucket using the supplied Google Cloud credentials.
    /// <summary>
    public class GoogleCloudStorageConfig
    {
        [JsonProperty("bucket", Required = Required.Always)]
        public required string Bucket { get; set; }

        [JsonProperty("credentials", Required = Required.Always)]
        public required string Credentials { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GoogleCloudStorageConfig {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append('\n');
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
