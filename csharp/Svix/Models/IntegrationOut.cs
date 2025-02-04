// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class IntegrationOut(DateTime updatedAt, string name, string id, DateTime createdAt)
        : BaseModel
    {
        [JsonPropertyName("createdAt")]
        public DateTime CreatedAt { get; set; } = createdAt;

        [JsonPropertyName("id")]
        public string Id { get; set; } = id;

        [JsonPropertyName("name")]
        public string Name { get; set; } = name;

        [JsonPropertyName("updatedAt")]
        public DateTime UpdatedAt { get; set; } = updatedAt;
    }
}
