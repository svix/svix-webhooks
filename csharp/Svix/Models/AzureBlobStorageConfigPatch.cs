// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AzureBlobStorageConfigPatch
    {
        [JsonProperty("container")]
        public string? Container { get; set; } = null;

        public bool ShouldSerializeContainer() => Container != null;

        [JsonProperty("account")]
        public string? Account { get; set; } = null;

        public bool ShouldSerializeAccount() => Account != null;

        [JsonProperty("accessKey")]
        public string? AccessKey { get; set; } = null;

        public bool ShouldSerializeAccessKey() => AccessKey != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AzureBlobStorageConfigPatch {\n");
            sb.Append("  Container: ").Append(Container).Append('\n');
            sb.Append("  Account: ").Append(Account).Append('\n');
            sb.Append("  AccessKey: ").Append(AccessKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
