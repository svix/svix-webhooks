// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ListResponseMessageAttemptOut(
        string? iterator,
        bool done,
        List<MessageAttemptOut> data,
        string? prevIterator = null
    ) : BaseModel
    {
        [JsonPropertyName("data")]
        public List<MessageAttemptOut> Data { get; set; } = data;

        [JsonPropertyName("done")]
        public bool Done { get; set; } = done;

        [JsonPropertyName("iterator")]
        public string? Iterator { get; set; } = iterator;

        [JsonPropertyName("prevIterator")]
        public string? PrevIterator { get; set; } = prevIterator;
    }
}
