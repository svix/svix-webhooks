// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RecoverIn : BaseModel
    {
        [JsonProperty("since")]
        public required DateTime Since { get; set; }

        [JsonProperty("until")]
        public DateTime? Until { get; set; } = null;
    }
}
