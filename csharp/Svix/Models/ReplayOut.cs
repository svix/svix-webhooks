// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ReplayOut : BaseModel
    {
        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("status")]
        public required BackgroundTaskStatus Status { get; set; }

        [JsonProperty("task")]
        public required BackgroundTaskType Task { get; set; }
    }
}
