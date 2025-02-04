// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointTransformationIn(bool? enabled = null, string? code = null) : BaseModel
    {
        [JsonPropertyName("code")]
        public string? Code { get; set; } = code;

        [JsonPropertyName("enabled")]
        public bool? Enabled { get; set; } = enabled;
    }
}
