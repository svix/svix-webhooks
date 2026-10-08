// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ElasticSearchConfigOut
    {
        [JsonProperty("indexName", Required = Required.Always)]
        public required string IndexName { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public required string Url { get; set; }

        [JsonProperty("refresh", Required = Required.Always)]
        public required bool Refresh { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ElasticSearchConfigOut {\n");
            sb.Append("  IndexName: ").Append(IndexName).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Refresh: ").Append(Refresh).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
