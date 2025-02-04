// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOutData(
        List<string> modified,
        List<EventTypeFromOpenApi>? toModify = null
    ) : BaseModel
    {
        [JsonPropertyName("modified")]
        public List<string> Modified { get; set; } = modified;

        [JsonPropertyName("to_modify")]
        public List<EventTypeFromOpenApi>? ToModify { get; set; } = toModify;
    }
}
