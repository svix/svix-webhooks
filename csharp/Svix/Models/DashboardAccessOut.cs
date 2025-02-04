// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class DashboardAccessOut(string url, string token) : BaseModel
    {
        [JsonPropertyName("token")]
        public string Token { get; set; } = token;

        [JsonPropertyName("url")]
        public string Url { get; set; } = url;
    }
}
