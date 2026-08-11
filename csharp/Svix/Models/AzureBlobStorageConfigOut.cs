// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AzureBlobStorageConfigOut
    {
        [JsonProperty("container", Required = Required.Always)]
        public required string Container { get; set; }

        [JsonProperty("account", Required = Required.Always)]
        public required string Account { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AzureBlobStorageConfigOut {\n");
            sb.Append("  Container: ").Append(Container).Append('\n');
            sb.Append("  Account: ").Append(Account).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
