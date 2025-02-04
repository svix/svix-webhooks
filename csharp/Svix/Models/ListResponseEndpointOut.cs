// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ListResponseEndpointOut(
        string? iterator,
        bool done,
        List<EndpointOut> data,
        string? prevIterator = null
    ) : BaseModel
    {
        [JsonPropertyName("data")]
        public List<EndpointOut> Data { get; set; } = data;

        [JsonPropertyName("done")]
        public bool Done { get; set; } = done;

        [JsonPropertyName("iterator")]
        public string? Iterator { get; set; } = iterator;

        [JsonPropertyName("prevIterator")]
        public string? PrevIterator { get; set; } = prevIterator;
    }
}
