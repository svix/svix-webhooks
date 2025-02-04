// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class AppUsageStatsIn(DateTime until, DateTime since, List<string>? appIds = null)
        : BaseModel
    {
        [JsonPropertyName("appIds")]
        public List<string>? AppIds { get; set; } = appIds;

        [JsonPropertyName("since")]
        public DateTime Since { get; set; } = since;

        [JsonPropertyName("until")]
        public DateTime Until { get; set; } = until;
    }
}
