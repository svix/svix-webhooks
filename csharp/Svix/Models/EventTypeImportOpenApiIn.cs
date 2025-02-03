// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    /// <summary>
    /// Import a list of event types from webhooks defined in an OpenAPI spec.
    ///
    /// The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
    /// <summary>

    public class EventTypeImportOpenApiIn : BaseModel
    {
        [JsonPropertyName("dryRun")]
        public bool? DryRun { get; set; }

        [JsonPropertyName("replaceAll")]
        public bool? ReplaceAll { get; set; }

        [JsonPropertyName("spec")]
        public Object? Spec { get; set; }

        [JsonPropertyName("specRaw")]
        public string? SpecRaw { get; set; }
    }
}
