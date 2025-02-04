// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class OperationalWebhookEndpointHeadersOut(
        List<string> sensitive,
        Dictionary<string, string> headers
    ) : BaseModel
    {
        [JsonPropertyName("headers")]
        public Dictionary<string, string> Headers { get; set; } = headers;

        [JsonPropertyName("sensitive")]
        public List<string> Sensitive { get; set; } = sensitive;
    }
}
