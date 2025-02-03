// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class OperationalWebhookEndpointHeadersOut : BaseModel
    {
        [JsonPropertyName("headers")]
        public required Dictionary<string, string> Headers { get; set; }

        [JsonPropertyName("sensitive")]
        public required List<string> Sensitive { get; set; }
    }
}
