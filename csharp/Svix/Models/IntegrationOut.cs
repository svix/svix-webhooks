// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class IntegrationOut : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public required DateTime CreatedAt { get; set; }

        [JsonPropertyName("id")]
        public required string Id { get; set; }

        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("updatedAt")]
        public required DateTime UpdatedAt { get; set; }
    }
}
