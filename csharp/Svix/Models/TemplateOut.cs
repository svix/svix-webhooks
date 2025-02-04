// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class TemplateOut : BaseModel
    {
        [JsonProperty("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("description")]
        public required string Description { get; set; }

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("instructions")]
        public required string Instructions { get; set; }

        [JsonProperty("instructionsLink")]
        public string? InstructionsLink { get; set; } = null;

        [JsonProperty("kind")]
        public required TransformationTemplateKind Kind { get; set; }

        [JsonProperty("logo")]
        public required string Logo { get; set; }

        [JsonProperty("name")]
        public required string Name { get; set; }

        [JsonProperty("orgId")]
        public required string OrgId { get; set; }

        [JsonProperty("transformation")]
        public required string Transformation { get; set; }

        [JsonProperty("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
