// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApiTokenExpireIn
    {
        [JsonProperty("expiry")]
        public int? Expiry { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ApiTokenExpireIn {\n");
            sb.Append("  Expiry: ").Append(Expiry).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
