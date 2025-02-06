// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationKeyOut : BaseModel
    {
        [JsonProperty("key", Required = Required.Always)]
        public required string Key { get; set; }
    }
}
