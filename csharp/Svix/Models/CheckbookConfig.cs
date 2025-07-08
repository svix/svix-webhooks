// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class CheckbookConfig
    {
        [JsonProperty("secret", Required = Required.Always)]
        public required string Secret { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class CheckbookConfig {\n");
            sb.Append("  Secret: ").Append(Secret).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
