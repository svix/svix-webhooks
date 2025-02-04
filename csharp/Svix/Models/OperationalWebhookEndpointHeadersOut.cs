// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class OperationalWebhookEndpointHeadersOut : BaseModel
    {
        [JsonProperty("headers")]
        public required Dictionary<string, string> Headers { get; set; }

        [JsonProperty("sensitive")]
        public required List<string> Sensitive { get; set; }
    }
}
