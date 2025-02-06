// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class DashboardAccessOut : BaseModel
    {
        [JsonProperty("token", Required = Required.Always)]
        public required string Token { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }
    }
}
