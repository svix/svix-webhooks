// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class IntegrationKeyOut(string key) : BaseModel
    {
        [JsonPropertyName("key")]
        public string Key { get; set; } = key;
    }
}
