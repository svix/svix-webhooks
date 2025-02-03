// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class TemplateOut : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonPropertyName("description")]
        public required string Description { get; set; }

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; }

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("instructions")]
        public required string Instructions { get; set; }

        [JsonPropertyName("instructionsLink")]
        public string? InstructionsLink { get; set; }

        [JsonPropertyName("kind")]
        public required TransformationTemplateKind Kind { get; set; }

        [JsonPropertyName("logo")]
        public required string Logo { get; set; }

        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("orgId")]
        public required string OrgId { get; set; }

        [JsonPropertyName("transformation")]
        public required string Transformation { get; set; }

        [JsonPropertyName("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
