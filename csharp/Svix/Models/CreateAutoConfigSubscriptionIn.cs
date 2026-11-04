// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class CreateAutoConfigSubscriptionIn
    {
        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class CreateAutoConfigSubscriptionIn {\n");
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
