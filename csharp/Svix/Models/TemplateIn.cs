// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class TemplateIn(
        string transformation,
        string name,
        string logo,
        TransformationTemplateKind? kind = null,
        string? instructionsLink = null,
        string? instructions = null,
        List<string>? filterTypes = null,
        string? featureFlag = null,
        string? description = null
    ) : BaseModel
    {
        [JsonPropertyName("description")]
        public string? Description { get; set; } = description;

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; } = featureFlag;

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; } = filterTypes;

        [JsonPropertyName("instructions")]
        public string? Instructions { get; set; } = instructions;

        [JsonPropertyName("instructionsLink")]
        public string? InstructionsLink { get; set; } = instructionsLink;

        [JsonPropertyName("kind")]
        public TransformationTemplateKind? Kind { get; set; } = kind;

        [JsonPropertyName("logo")]
        public string Logo { get; set; } = logo;

        [JsonPropertyName("name")]
        public string Name { get; set; } = name;

        [JsonPropertyName("transformation")]
        public string Transformation { get; set; } = transformation;
    }
}
