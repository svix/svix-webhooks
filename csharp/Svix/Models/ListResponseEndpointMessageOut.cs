// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ListResponseEndpointMessageOut(
        string? iterator,
        bool done,
        List<EndpointMessageOut> data,
        string? prevIterator = null
    ) : BaseModel
    {
        [JsonPropertyName("data")]
        public List<EndpointMessageOut> Data { get; set; } = data;

        [JsonPropertyName("done")]
        public bool Done { get; set; } = done;

        [JsonPropertyName("iterator")]
        public string? Iterator { get; set; } = iterator;

        [JsonPropertyName("prevIterator")]
        public string? PrevIterator { get; set; } = prevIterator;
    }
}
