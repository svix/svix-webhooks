// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class AppPortalAccessIn : BaseModel
    {
        [JsonPropertyName("application")]
        public ApplicationIn? Application { get; set; }

        [JsonPropertyName("expiry")]
        public ulong? Expiry { get; set; }

        [JsonPropertyName("featureFlags")]
        public List<string>? FeatureFlags { get; set; }

        [JsonPropertyName("readOnly")]
        public bool? ReadOnly { get; set; }
    }
}
