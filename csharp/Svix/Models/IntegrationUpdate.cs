// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class IntegrationUpdate(string name) : BaseModel
    {
        [JsonPropertyName("name")]
        public string Name { get; set; } = name;
    }
}
