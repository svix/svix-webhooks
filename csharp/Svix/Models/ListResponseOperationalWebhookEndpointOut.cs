// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class ListResponseOperationalWebhookEndpointOut : BaseModel
    {
        [JsonPropertyName("data")]
        public required List<OperationalWebhookEndpointOut> Data { get; set; }

        [JsonPropertyName("done")]
        public required bool Done { get; set; }

        [JsonPropertyName("iterator")]
        public required string? Iterator { get; set; }

        [JsonPropertyName("prevIterator")]
        public string? PrevIterator { get; set; }
    }
}
