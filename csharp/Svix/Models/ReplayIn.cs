// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ReplayIn(DateTime since, DateTime? until = null) : BaseModel
    {
        [JsonPropertyName("since")]
        public DateTime Since { get; set; } = since;

        [JsonPropertyName("until")]
        public DateTime? Until { get; set; } = until;
    }
}
