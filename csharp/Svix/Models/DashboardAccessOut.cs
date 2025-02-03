// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class DashboardAccessOut : BaseModel
    {
        [JsonPropertyName("token")]
        public required string Token { get; set; }

        [JsonPropertyName("url")]
        public required string Url { get; set; }
    }
}
