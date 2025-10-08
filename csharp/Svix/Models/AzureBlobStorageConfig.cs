// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AzureBlobStorageConfig
    {
        [JsonProperty("accessKey", Required = Required.Always)]
        public required string AccessKey { get; set; }

        [JsonProperty("account", Required = Required.Always)]
        public required string Account { get; set; }

        [JsonProperty("container", Required = Required.Always)]
        public required string Container { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AzureBlobStorageConfig {\n");
            sb.Append("  AccessKey: ").Append(AccessKey).Append('\n');
            sb.Append("  Account: ").Append(Account).Append('\n');
            sb.Append("  Container: ").Append(Container).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
