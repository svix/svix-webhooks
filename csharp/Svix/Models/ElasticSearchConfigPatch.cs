// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ElasticSearchConfigPatch
    {
        [JsonProperty("indexName")]
        public string? IndexName { get; set; } = null;

        public bool ShouldSerializeIndexName() => IndexName != null;

        [JsonProperty("url")]
        public string? Url { get; set; } = null;

        public bool ShouldSerializeUrl() => Url != null;

        [JsonProperty("apiKey")]
        public string? ApiKey { get; set; } = null;

        public bool ShouldSerializeApiKey() => ApiKey != null;

        [JsonProperty("refresh")]
        public bool? Refresh { get; set; } = null;

        public bool ShouldSerializeRefresh() => Refresh != null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ElasticSearchConfigPatch {\n");
            sb.Append("  IndexName: ").Append(IndexName).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  ApiKey: ").Append(ApiKey).Append('\n');
            sb.Append("  Refresh: ").Append(Refresh).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
