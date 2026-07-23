// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class StreamEventTypeOut
    {
        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        [JsonProperty("deprecated", Required = Required.Always)]
        public required bool Deprecated { get; set; }

        [JsonProperty("archived", Required = Required.Always)]
        public required bool Archived { get; set; }

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class StreamEventTypeOut {\n");
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  Deprecated: ").Append(Deprecated).Append('\n');
            sb.Append("  Archived: ").Append(Archived).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
