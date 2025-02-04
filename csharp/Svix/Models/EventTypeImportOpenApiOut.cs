// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOut(EventTypeImportOpenApiOutData data) : BaseModel
    {
        [JsonPropertyName("data")]
        public EventTypeImportOpenApiOutData Data { get; set; } = data;
    }
}
