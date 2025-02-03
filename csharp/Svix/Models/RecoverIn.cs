// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class RecoverIn : BaseModel
    {
        [JsonPropertyName("since")]
        public required DateTime Since { get; set; }

        [JsonPropertyName("until")]
        public DateTime? Until { get; set; }
    }
}
