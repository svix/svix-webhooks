// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration parameters for defining a Segment sink.
    /// <summary>
    public class SegmentSinkConfigIn
    {
        [JsonProperty("host", Required = Required.Always)]
        public required string Host { get; set; }

        [JsonProperty("writeKey", Required = Required.Always)]
        public required string WriteKey { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SegmentSinkConfigIn {\n");
            sb.Append("  Host: ").Append(Host).Append('\n');
            sb.Append("  WriteKey: ").Append(WriteKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
