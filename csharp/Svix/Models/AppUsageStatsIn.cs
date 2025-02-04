// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppUsageStatsIn : BaseModel
    {
        [JsonProperty("appIds")]
        public List<string>? AppIds { get; set; } = null;

        [JsonProperty("since")]
        public required DateTime Since { get; set; }

        [JsonProperty("until")]
        public required DateTime Until { get; set; }
    }
}
