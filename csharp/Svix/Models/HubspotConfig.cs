// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class HubspotConfig
    {
        [JsonProperty("secret")]
        public string? Secret { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class HubspotConfig {\n");
            sb.Append("  Secret: ").Append(Secret).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
