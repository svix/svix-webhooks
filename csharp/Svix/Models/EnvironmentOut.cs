// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EnvironmentOut : BaseModel
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("eventTypes", Required = Required.Always)]
        public required List<EventTypeOut> EventTypes { get; set; }

        [JsonProperty("settings")]
        public Object? Settings { get; set; } = null;

        [JsonProperty("transformationTemplates", Required = Required.Always)]
        public required List<TemplateOut> TransformationTemplates { get; set; }

        [JsonProperty("version")]
        public long? Version { get; set; } = null;
    }
}
