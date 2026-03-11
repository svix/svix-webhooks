// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class StreamTokenExpireIn
    {
        [JsonProperty("expiry")]
        public long? Expiry { get; set; } = null;

        [JsonProperty("sessionIds")]
        public List<string>? SessionIds { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class StreamTokenExpireIn {\n");
            sb.Append("  Expiry: ").Append(Expiry).Append('\n');
            sb.Append("  SessionIds: ").Append(SessionIds).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
