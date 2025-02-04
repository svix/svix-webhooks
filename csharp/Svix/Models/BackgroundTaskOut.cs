// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class BackgroundTaskOut(
        BackgroundTaskType task,
        BackgroundTaskStatus status,
        string id,
        Object data
    ) : BaseModel
    {
        [JsonPropertyName("data")]
        public Object Data { get; set; } = data;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("status")]
        public BackgroundTaskStatus Status { get; set; } = status;

        [JsonPropertyName("task")]
        public BackgroundTaskType Task { get; set; } = task;
    }
}
