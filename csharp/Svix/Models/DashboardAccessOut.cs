// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class DashboardAccessOut : BaseModel
    {
        [JsonProperty("token")]
        public required string Token { get; set; }

        [JsonProperty("url")]
        public required string Url { get; set; }
    }
}
