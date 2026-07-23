// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GoogleCloudPubSubPatchConfig
    {
        [JsonProperty("projectId")]
        public string? ProjectId { get; set; } = null;

        [JsonProperty("topicId")]
        public string? TopicId { get; set; } = null;

        [JsonProperty("credentials")]
        public string? Credentials { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GoogleCloudPubSubPatchConfig {\n");
            sb.Append("  ProjectId: ").Append(ProjectId).Append('\n');
            sb.Append("  TopicId: ").Append(TopicId).Append('\n');
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
