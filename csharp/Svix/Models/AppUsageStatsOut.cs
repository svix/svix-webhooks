// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class AppUsageStatsOut(
        List<string> unresolvedAppIds,
        BackgroundTaskType task,
        BackgroundTaskStatus status,
        string id
    ) : BaseModel
    {
        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("status")]
        public BackgroundTaskStatus Status { get; set; } = status;

        [JsonPropertyName("task")]
        public BackgroundTaskType Task { get; set; } = task;

        [JsonPropertyName("unresolvedAppIds")]
        public List<string> UnresolvedAppIds { get; set; } = unresolvedAppIds;
    }
}
