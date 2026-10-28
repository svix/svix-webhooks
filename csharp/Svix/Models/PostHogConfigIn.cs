// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration parameters for defining a PostHog sink.
    /// <summary>
    public class PostHogConfigIn
    {
        [JsonProperty("domain", Required = Required.Always)]
        public required string Domain { get; set; }

        [JsonProperty("projectToken", Required = Required.Always)]
        public required string ProjectToken { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PostHogConfigIn {\n");
            sb.Append("  Domain: ").Append(Domain).Append('\n');
            sb.Append("  ProjectToken: ").Append(ProjectToken).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
