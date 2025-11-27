// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ConnectorIn
    {
        [JsonProperty("allowedEventTypes")]
        public List<string>? AllowedEventTypes { get; set; } = null;

        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("instructions")]
        public string? Instructions { get; set; } = null;

        [JsonProperty("kind")]
        public ConnectorKind? Kind { get; set; } = null;

        [JsonProperty("logo")]
        public string? Logo { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("productType")]
        public ConnectorProduct? ProductType { get; set; } = null;

        [JsonProperty("transformation", Required = Required.Always)]
        public required string Transformation { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ConnectorIn {\n");
            sb.Append("  AllowedEventTypes: ").Append(AllowedEventTypes).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  Instructions: ").Append(Instructions).Append('\n');
            sb.Append("  Kind: ").Append(Kind).Append('\n');
            sb.Append("  Logo: ").Append(Logo).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  ProductType: ").Append(ProductType).Append('\n');
            sb.Append("  Transformation: ").Append(Transformation).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
