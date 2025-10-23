// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ConnectorPatch
    {
        [JsonProperty("allowedEventTypes")]
        public MaybeUnset<List<string>?> AllowedEventTypes { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeAllowedEventTypes() => !AllowedEventTypes.IsUnset;

        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        public bool ShouldSerializeDescription() => Description != null;

        [JsonProperty("featureFlags")]
        public MaybeUnset<List<string>?> FeatureFlags { get; set; } =
            MaybeUnset<List<string>?>.Unset();

        public bool ShouldSerializeFeatureFlags() => !FeatureFlags.IsUnset;

        [JsonProperty("instructions")]
        public string? Instructions { get; set; } = null;

        public bool ShouldSerializeInstructions() => Instructions != null;

        [JsonProperty("kind")]
        public ConnectorKind? Kind { get; set; } = null;

        public bool ShouldSerializeKind() => Kind != null;

        [JsonProperty("logo")]
        public MaybeUnset<string?> Logo { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeLogo() => !Logo.IsUnset;

        [JsonProperty("name")]
        public string? Name { get; set; } = null;

        public bool ShouldSerializeName() => Name != null;

        [JsonProperty("transformation")]
        public string? Transformation { get; set; } = null;

        public bool ShouldSerializeTransformation() => Transformation != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ConnectorPatch {\n");
            sb.Append("  AllowedEventTypes: ").Append(AllowedEventTypes).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  Instructions: ").Append(Instructions).Append('\n');
            sb.Append("  Kind: ").Append(Kind).Append('\n');
            sb.Append("  Logo: ").Append(Logo).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Transformation: ").Append(Transformation).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
