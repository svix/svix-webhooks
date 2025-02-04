// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class OperationalWebhookEndpointHeadersIn(Dictionary<string, string> headers) : BaseModel
    {
        [JsonPropertyName("headers")]
        public Dictionary<string, string> Headers { get; set; } = headers;
    }
}
