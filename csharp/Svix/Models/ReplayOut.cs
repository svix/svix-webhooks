// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ReplayOut : BaseModel
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required BackgroundTaskStatus Status { get; set; }

        [JsonProperty("task", Required = Required.Always)]
        public required BackgroundTaskType Task { get; set; }
    }
}
