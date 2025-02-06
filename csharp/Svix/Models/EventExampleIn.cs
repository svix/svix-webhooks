// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventExampleIn : BaseModel
    {
        [JsonProperty("eventType", Required = Required.Always)]
        public required string EventType { get; set; }

        [JsonProperty("exampleIndex")]
        public ulong? ExampleIndex { get; set; } = null;
    }
}
