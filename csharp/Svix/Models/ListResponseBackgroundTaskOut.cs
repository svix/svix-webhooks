// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ListResponseBackgroundTaskOut : BaseModel
    {
        [JsonProperty("data")]
        public required List<BackgroundTaskOut> Data { get; set; }

        [JsonProperty("done")]
        public required bool Done { get; set; }

        [JsonProperty("iterator")]
        public string? Iterator { get; set; } = null;

        [JsonProperty("prevIterator")]
        public string? PrevIterator { get; set; } = null;
    }
}
