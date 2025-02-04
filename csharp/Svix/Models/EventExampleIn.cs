// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EventExampleIn(string eventType, ulong? exampleIndex = null) : BaseModel
    {
        [JsonPropertyName("eventType")]
        public string EventType { get; set; } = eventType;

        [JsonPropertyName("exampleIndex")]
        public ulong? ExampleIndex { get; set; } = exampleIndex;
    }
}
