// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ApplicationTokenExpireIn(long? expiry = null) : BaseModel
    {
        [JsonPropertyName("expiry")]
        public long? Expiry { get; set; } = expiry;
    }
}
