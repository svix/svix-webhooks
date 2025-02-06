// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOut : BaseModel
    {
        [JsonProperty("data", Required = Required.Always)]
        public required EventTypeImportOpenApiOutData Data { get; set; }
    }
}
