// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ListResponseEndpointMessageOut : BaseModel
    {
        [JsonProperty("data")]
        public required List<EndpointMessageOut> Data { get; set; }

        [JsonProperty("done")]
        public required bool Done { get; set; }

        [JsonProperty("iterator")]
        public string? Iterator { get; set; } = null;

        [JsonProperty("prevIterator")]
        public string? PrevIterator { get; set; } = null;
    }
}
