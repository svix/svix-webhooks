// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EnvironmentOut : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonPropertyName("eventTypes")]
        public required List<EventTypeOut> EventTypes { get; set; }

        [JsonPropertyName("settings")]
        public required Object? Settings { get; set; }

        [JsonPropertyName("transformationTemplates")]
        public required List<TemplateOut> TransformationTemplates { get; set; }

        [JsonPropertyName("version")]
        public long? Version { get; set; }
    }
}
