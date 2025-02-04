// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EnvironmentIn(
        List<TemplateIn>? transformationTemplates = null,
        Object? settings = null,
        List<EventTypeIn>? eventTypes = null
    ) : BaseModel
    {
        [JsonPropertyName("eventTypes")]
        public List<EventTypeIn>? EventTypes { get; set; } = eventTypes;

        [JsonPropertyName("settings")]
        public Object? Settings { get; set; } = settings;

        [JsonPropertyName("transformationTemplates")]
        public List<TemplateIn>? TransformationTemplates { get; set; } = transformationTemplates;
    }
}
