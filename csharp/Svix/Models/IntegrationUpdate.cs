// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationUpdate : BaseModel
    {
        [JsonProperty("name", Required = Required.Always)]
        public required string Name { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IntegrationUpdate {\n");
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
