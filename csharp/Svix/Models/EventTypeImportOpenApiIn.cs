// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Import a list of event types from webhooks defined in an OpenAPI spec.
    ///
    /// The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
    /// <summary>
    public class EventTypeImportOpenApiIn
    {
        [JsonProperty("dryRun")]
        public bool? DryRun { get; set; } = null;

        [JsonProperty("replaceAll")]
        public bool? ReplaceAll { get; set; } = null;

        [JsonProperty("spec")]
        public Object? Spec { get; set; } = null;

        [JsonProperty("specRaw")]
        public string? SpecRaw { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventTypeImportOpenApiIn {\n");
            sb.Append("  DryRun: ").Append(DryRun).Append('\n');
            sb.Append("  ReplaceAll: ").Append(ReplaceAll).Append('\n');
            sb.Append("  Spec: ").Append(Spec).Append('\n');
            sb.Append("  SpecRaw: ").Append(SpecRaw).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
