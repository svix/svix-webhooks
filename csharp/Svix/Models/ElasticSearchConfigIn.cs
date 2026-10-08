// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Configuration parameters for defining an ElasticSearch/OpenSearch sink.
    /// <summary>
    public class ElasticSearchConfigIn
    {
        [JsonProperty("indexName", Required = Required.Always)]
        public required string IndexName { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("apiKey")]
        public string? ApiKey { get; set; } = null;

        [JsonProperty("refresh")]
        public bool? Refresh { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ElasticSearchConfigIn {\n");
            sb.Append("  IndexName: ").Append(IndexName).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  ApiKey: ").Append(ApiKey).Append('\n');
            sb.Append("  Refresh: ").Append(Refresh).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
