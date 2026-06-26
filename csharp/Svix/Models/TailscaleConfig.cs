// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class TailscaleConfig
    {
        [JsonProperty("secret", Required = Required.Always)]
        public required string Secret { get; set; }

        [JsonProperty("timestampGraceSeconds")]
        public uint? TimestampGraceSeconds { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class TailscaleConfig {\n");
            sb.Append("  Secret: ").Append(Secret).Append('\n');
            sb.Append("  TimestampGraceSeconds: ").Append(TimestampGraceSeconds).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
