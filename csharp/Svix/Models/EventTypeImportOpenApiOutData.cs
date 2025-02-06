// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOutData : BaseModel
    {
        [JsonProperty("modified", Required = Required.Always)]
        public required List<string> Modified { get; set; }

        [JsonProperty("to_modify")]
        public List<EventTypeFromOpenApi>? ToModify { get; set; } = null;
    }
}
