// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class SegmentSinkConfigPatch
    {
        [JsonProperty("host")]
        public string? Host { get; set; } = null;

        public bool ShouldSerializeHost() => Host != null;

        [JsonProperty("writeKey")]
        public string? WriteKey { get; set; } = null;

        public bool ShouldSerializeWriteKey() => WriteKey != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class SegmentSinkConfigPatch {\n");
            sb.Append("  Host: ").Append(Host).Append('\n');
            sb.Append("  WriteKey: ").Append(WriteKey).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
