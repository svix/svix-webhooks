// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointHeadersIn : BaseModel
    {
        [JsonPropertyName("headers")]
        public required Dictionary<string, string> Headers { get; set; }
    }
}
