// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EnvironmentIn : BaseModel
    {
        [JsonPropertyName("eventTypes")]
        public List<EventTypeIn>? EventTypes { get; set; }

        [JsonPropertyName("settings")]
        public Object? Settings { get; set; }

        [JsonPropertyName("transformationTemplates")]
        public List<TemplateIn>? TransformationTemplates { get; set; }
    }
}
