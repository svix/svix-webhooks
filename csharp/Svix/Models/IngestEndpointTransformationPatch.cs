// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IngestEndpointTransformationPatch
    {
        [JsonProperty("code")]
        public MaybeUnset<string?> Code { get; set; } = MaybeUnset<string?>.Unset();

        public bool ShouldSerializeCode() => !Code.IsUnset;

        [JsonProperty("enabled")]
        public bool? Enabled { get; set; } = null;

        public bool ShouldSerializeEnabled() => Enabled != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestEndpointTransformationPatch {\n");
            sb.Append("  Code: ").Append(Code).Append('\n');
            sb.Append("  Enabled: ").Append(Enabled).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
