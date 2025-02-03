// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOutData : BaseModel
    {
        [JsonPropertyName("modified")]
        public required List<string> Modified { get; set; }

        [JsonPropertyName("to_modify")]
        public List<EventTypeFromOpenApi>? ToModify { get; set; }
    }
}
