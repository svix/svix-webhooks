// this file is @generated
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationUpdate : BaseModel
    {
        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }
    }
}
