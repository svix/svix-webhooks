// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class TemplateIn : BaseModel
    {
        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("instructions")]
        public string? Instructions { get; set; } = null;

        [JsonProperty("instructionsLink")]
        public string? InstructionsLink { get; set; } = null;

        [JsonProperty("kind")]
        public TransformationTemplateKind? Kind { get; set; } = null;

        [JsonProperty("logo", Required = Required.Always)]
        public required string Logo { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("transformation", Required = Required.Always)]
        public required string Transformation { get; set; }
    }
}
