// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ConnectorOut
    {
        [JsonProperty("allowedEventTypes")]
        public List<string>? AllowedEventTypes { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public required string Description { get; set; }

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("instructions", Required = Required.Always)]
        public required string Instructions { get; set; }

        [JsonProperty("kind", Required = Required.Always)]
        public required ConnectorKind Kind { get; set; }

        [JsonProperty("logo")]
        public string? Logo { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("orgId", Required = Required.Always)]
        public required string OrgId { get; set; }

        [JsonProperty("productType", Required = Required.Always)]
        public required ConnectorProduct ProductType { get; set; }

        [JsonProperty("transformation", Required = Required.Always)]
        public required string Transformation { get; set; }

        [JsonProperty("transformationUpdatedAt", Required = Required.Always)]
        public required DateTime TransformationUpdatedAt { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ConnectorOut {\n");
            sb.Append("  AllowedEventTypes: ").Append(AllowedEventTypes).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Instructions: ").Append(Instructions).Append('\n');
            sb.Append("  Kind: ").Append(Kind).Append('\n');
            sb.Append("  Logo: ").Append(Logo).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  OrgId: ").Append(OrgId).Append('\n');
            sb.Append("  ProductType: ").Append(ProductType).Append('\n');
            sb.Append("  Transformation: ").Append(Transformation).Append('\n');
            sb.Append("  TransformationUpdatedAt: ").Append(TransformationUpdatedAt).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
