// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EnvironmentOut : BaseModel
    {
        [JsonProperty("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("eventTypes")]
        public required List<EventTypeOut> EventTypes { get; set; }

        [JsonProperty("settings")]
        public Object? Settings { get; set; } = null;

        [JsonProperty("transformationTemplates")]
        public required List<TemplateOut> TransformationTemplates { get; set; }

        [JsonProperty("version")]
        public long? Version { get; set; } = null;
    }
}
