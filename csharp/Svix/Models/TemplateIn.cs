// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class TemplateIn : BaseModel
    {
        [JsonPropertyName("description")]
        public string? Description { get; set; }

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; }

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; }

        [JsonPropertyName("instructions")]
        public string? Instructions { get; set; }

        [JsonPropertyName("instructionsLink")]
        public string? InstructionsLink { get; set; }

        [JsonPropertyName("kind")]
        public TransformationTemplateKind? Kind { get; set; }

        [JsonPropertyName("logo")]
        public required string Logo { get; set; }

        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("transformation")]
        public required string Transformation { get; set; }
    }
}
