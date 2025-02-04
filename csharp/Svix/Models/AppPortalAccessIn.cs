// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class AppPortalAccessIn(
        bool? readOnly = null,
        List<string>? featureFlags = null,
        ulong? expiry = null,
        ApplicationIn? application = null
    ) : BaseModel
    {
        [JsonPropertyName("application")]
        public ApplicationIn? Application { get; set; } = application;

        [JsonPropertyName("expiry")]
        public ulong? Expiry { get; set; } = expiry;

        [JsonPropertyName("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = featureFlags;

        [JsonPropertyName("readOnly")]
        public bool? ReadOnly { get; set; } = readOnly;
    }
}
