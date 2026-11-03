// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GenericConfig
    {
        [JsonProperty("response")]
        public GenericResponse? Response { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GenericConfig {\n");
            sb.Append("  Response: ").Append(Response).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
