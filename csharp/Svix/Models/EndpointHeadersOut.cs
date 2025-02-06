// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// The value of the headers is returned in the `headers` field.
    ///
    /// Sensitive headers that have been redacted are returned in the sensitive field.
    /// <summary>

    public class EndpointHeadersOut : BaseModel
    {
        [JsonProperty("headers", Required = Required.Always)]
        public required Dictionary<string, string> Headers { get; set; }

        [JsonProperty("sensitive", Required = Required.Always)]
        public required List<string> Sensitive { get; set; }
    }
}
