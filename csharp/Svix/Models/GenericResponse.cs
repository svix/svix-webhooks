// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GenericResponse
    {
        [JsonProperty("contentType", Required = Required.Always)]
        public required string ContentType { get; set; }

        [JsonProperty("body", Required = Required.Always)]
        public required string Body { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GenericResponse {\n");
            sb.Append("  ContentType: ").Append(ContentType).Append('\n');
            sb.Append("  Body: ").Append(Body).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
