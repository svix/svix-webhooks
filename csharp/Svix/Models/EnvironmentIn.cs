// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EnvironmentIn : BaseModel
    {
        [JsonProperty("eventTypes")]
        public List<EventTypeIn>? EventTypes { get; set; } = null;

        [JsonProperty("settings")]
        public Object? Settings { get; set; } = null;

        [JsonProperty("transformationTemplates")]
        public List<TemplateIn>? TransformationTemplates { get; set; } = null;
    }
}
