// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppPortalAccessIn : BaseModel
    {
        [JsonProperty("application")]
        public ApplicationIn? Application { get; set; } = null;

        [JsonProperty("expiry")]
        public ulong? Expiry { get; set; } = null;

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("readOnly")]
        public bool? ReadOnly { get; set; } = null;
    }
}
