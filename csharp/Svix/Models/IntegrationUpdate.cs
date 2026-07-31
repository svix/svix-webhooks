// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationUpdate
    {
        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IntegrationUpdate {\n");
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
