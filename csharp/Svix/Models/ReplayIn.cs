// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ReplayIn : BaseModel
    {
        [JsonPropertyName("since")]
        public required DateTime Since { get; set; }

        [JsonPropertyName("until")]
        public DateTime? Until { get; set; }
    }
}
