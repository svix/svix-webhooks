// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppUsageStatsIn : BaseModel
    {
        [JsonProperty("appIds")]
        public List<string>? AppIds { get; set; } = null;

        [JsonProperty("since", Required = Required.Always)]
        public required DateTime Since { get; set; }

        [JsonProperty("until", Required = Required.Always)]
        public required DateTime Until { get; set; }
    }
}
