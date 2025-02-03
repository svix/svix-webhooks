// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointTransformationIn : BaseModel
    {
        [JsonPropertyName("code")]
        public string? Code { get; set; }

        [JsonPropertyName("enabled")]
        public bool? Enabled { get; set; }
    }
}
