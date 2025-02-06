// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationOut : BaseModel
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public required DateTime CreatedAt { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }
    }
}
