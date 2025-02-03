// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class AppUsageStatsIn : BaseModel
    {
        [JsonPropertyName("appIds")]
        public List<string>? AppIds { get; set; }

        [JsonPropertyName("since")]
        public required DateTime Since { get; set; }

        [JsonPropertyName("until")]
        public required DateTime Until { get; set; }
    }
}
