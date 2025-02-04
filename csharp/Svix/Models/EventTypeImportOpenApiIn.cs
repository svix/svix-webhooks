// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    /// <summary>
    /// Import a list of event types from webhooks defined in an OpenAPI spec.
    ///
    /// The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
    /// <summary>

    public class EventTypeImportOpenApiIn(
        string? specRaw = null,
        Object? spec = null,
        bool? replaceAll = null,
        bool? dryRun = null
    ) : BaseModel
    {
        [JsonPropertyName("dryRun")]
        public bool? DryRun { get; set; } = dryRun;

        [JsonPropertyName("replaceAll")]
        public bool? ReplaceAll { get; set; } = replaceAll;

        [JsonPropertyName("spec")]
        public Object? Spec { get; set; } = spec;

        [JsonPropertyName("specRaw")]
        public string? SpecRaw { get; set; } = specRaw;
    }
}
