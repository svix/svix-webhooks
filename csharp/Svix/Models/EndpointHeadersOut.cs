// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    /// <summary>
    /// The value of the headers is returned in the `headers` field.
    ///
    /// Sensitive headers that have been redacted are returned in the sensitive field.
    /// <summary>

    public class EndpointHeadersOut(List<string> sensitive, Dictionary<string, string> headers)
        : BaseModel
    {
        [JsonPropertyName("headers")]
        public Dictionary<string, string> Headers { get; set; } = headers;

        [JsonPropertyName("sensitive")]
        public List<string> Sensitive { get; set; } = sensitive;
    }
}
