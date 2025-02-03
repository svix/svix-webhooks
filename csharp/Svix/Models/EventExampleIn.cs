// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventExampleIn : BaseModel
    {
        [JsonPropertyName("eventType")]
        public required string EventType { get; set; }

        [JsonPropertyName("exampleIndex")]
        public ulong? ExampleIndex { get; set; }
    }
}
