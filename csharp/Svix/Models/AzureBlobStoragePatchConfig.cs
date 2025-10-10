// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AzureBlobStoragePatchConfig
    {
        [JsonProperty("accessKey")]
        public string? AccessKey { get; set; } = null;

        [JsonProperty("account")]
        public string? Account { get; set; } = null;

        [JsonProperty("container")]
        public string? Container { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AzureBlobStoragePatchConfig {\n");
            sb.Append("  AccessKey: ").Append(AccessKey).Append('\n');
            sb.Append("  Account: ").Append(Account).Append('\n');
            sb.Append("  Container: ").Append(Container).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
