// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class RecoverOut : BaseModel
    {
        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("status")]
        public required BackgroundTaskStatus Status { get; set; }

        [JsonPropertyName("task")]
        public required BackgroundTaskType Task { get; set; }
    }
}
