// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationOut : BaseModel
    {
        [JsonProperty("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("id")]
        public required string Id { get; set; }

        [JsonProperty("name")]
        public required string Name { get; set; }

        [JsonProperty("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
