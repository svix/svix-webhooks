// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class OperationalWebhookEndpointHeadersIn : BaseModel
    {
        [JsonProperty("headers")]
        public required Dictionary<string, string> Headers { get; set; }
    }
}
