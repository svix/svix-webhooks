// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointAttemptStats
    {
        [JsonProperty("success", Required = Required.Always)]
        public required ulong Success { get; set; }

        [JsonProperty("fail", Required = Required.Always)]
        public required ulong Fail { get; set; }

        [JsonProperty("canceled", Required = Required.Always)]
        public required ulong Canceled { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointAttemptStats {\n");
            sb.Append("  Success: ").Append(Success).Append('\n');
            sb.Append("  Fail: ").Append(Fail).Append('\n');
            sb.Append("  Canceled: ").Append(Canceled).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
