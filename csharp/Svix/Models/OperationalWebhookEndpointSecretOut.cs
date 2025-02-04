// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class OperationalWebhookEndpointSecretOut(string key) : BaseModel
    {
        [JsonPropertyName("key")]
        public string Key { get; set; } = key;
    }
}
