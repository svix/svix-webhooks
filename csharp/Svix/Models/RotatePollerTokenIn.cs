// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RotatePollerTokenIn
    {
        [JsonProperty("expiry")]
        public long? Expiry { get; set; } = null;

        [JsonProperty("oldTokenExpiry")]
        public long? OldTokenExpiry { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RotatePollerTokenIn {\n");
            sb.Append("  Expiry: ").Append(Expiry).Append('\n');
            sb.Append("  OldTokenExpiry: ").Append(OldTokenExpiry).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
