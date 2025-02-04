// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EnvironmentOut(
        List<TemplateOut> transformationTemplates,
        Object? settings,
        List<EventTypeOut> eventTypes,
        DateTime createdAt,
        long? version = null
    ) : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public DateTime CreatedAt { get; set; } = createdAt;

        [JsonPropertyName("eventTypes")]
        public List<EventTypeOut> EventTypes { get; set; } = eventTypes;

        [JsonPropertyName("settings")]
        public Object? Settings { get; set; } = settings;

        [JsonPropertyName("transformationTemplates")]
        public List<TemplateOut> TransformationTemplates { get; set; } = transformationTemplates;

        [JsonPropertyName("version")]
        public long? Version { get; set; } = version;
    }
}
