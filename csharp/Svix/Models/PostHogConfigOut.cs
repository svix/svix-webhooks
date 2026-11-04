// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PostHogConfigOut
    {
        [JsonProperty("domain", Required = Required.Always)]
        public required string Domain { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PostHogConfigOut {\n");
            sb.Append("  Domain: ").Append(Domain).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
