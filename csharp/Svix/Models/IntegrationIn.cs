// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class IntegrationIn(string name) : BaseModel
    {
        [JsonPropertyName("name")]
        public string Name { get; set; } = name;
    }
}
