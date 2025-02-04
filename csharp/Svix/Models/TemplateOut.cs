// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class TemplateOut(
        DateTime updatedAt,
        string transformation,
        string orgId,
        string name,
        string logo,
        TransformationTemplateKind kind,
        string instructions,
        string id,
        string description,
        DateTime createdAt,
        string? instructionsLink = null,
        List<string>? filterTypes = null,
        string? featureFlag = null
    ) : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public DateTime CreatedAt { get; set; } = createdAt;

        [JsonPropertyName("description")]
        public string Description { get; set; } = description;

        [JsonPropertyName("featureFlag")]
        public string? FeatureFlag { get; set; } = featureFlag;

        [JsonPropertyName("filterTypes")]
        public List<string>? FilterTypes { get; set; } = filterTypes;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("instructions")]
        public string Instructions { get; set; } = instructions;

        [JsonPropertyName("instructionsLink")]
        public string? InstructionsLink { get; set; } = instructionsLink;

        [JsonPropertyName("kind")]
        public TransformationTemplateKind Kind { get; set; } = kind;

        [JsonPropertyName("logo")]
        public string Logo { get; set; } = logo;

        [JsonPropertyName("name")]
        public string Name { get; set; } = name;

        [JsonPropertyName("orgId")]
        public string OrgId { get; set; } = orgId;

        [JsonPropertyName("transformation")]
        public string Transformation { get; set; } = transformation;

        [JsonPropertyName("updatedAt")]
        public DateTime UpdatedAt { get; set; } = updatedAt;
    }
}
