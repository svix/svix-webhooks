// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GenericWebhookConfig
    {
        [JsonProperty("response")]
        public GenericWebhookResponse? Response { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GenericWebhookConfig {\n");
            sb.Append("  Response: ").Append(Response).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
