// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class TemplateOut : BaseModel
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public required string Description { get; set; }

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("instructions", Required = Required.Always)]
        public required string Instructions { get; set; }

        [JsonProperty("instructionsLink")]
        public string? InstructionsLink { get; set; } = null;

        [JsonProperty("kind", Required = Required.Always)]
        public required TransformationTemplateKind Kind { get; set; }

        [JsonProperty("logo", Required = Required.Always)]
        public required string Logo { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("orgId", Required = Required.Always)]
        public required string OrgId { get; set; }

        [JsonProperty("transformation", Required = Required.Always)]
        public required string Transformation { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }
    }
}
