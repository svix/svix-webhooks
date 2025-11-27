// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class HttpAttemptTimes
    {
        [JsonProperty("end", Required = Required.Always)]
        public required DateTime End { get; set; }

        [JsonProperty("start", Required = Required.Always)]
        public required DateTime Start { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class HttpAttemptTimes {\n");
            sb.Append("  End: ").Append(End).Append('\n');
            sb.Append("  Start: ").Append(Start).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
