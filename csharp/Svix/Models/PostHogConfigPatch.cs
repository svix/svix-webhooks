// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PostHogConfigPatch
    {
        [JsonProperty("domain")]
        public string? Domain { get; set; } = null;

        public bool ShouldSerializeDomain() => Domain != null;

        [JsonProperty("projectToken")]
        public string? ProjectToken { get; set; } = null;

        public bool ShouldSerializeProjectToken() => ProjectToken != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PostHogConfigPatch {\n");
            sb.Append("  Domain: ").Append(Domain).Append('\n');
            sb.Append("  ProjectToken: ").Append(ProjectToken).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
