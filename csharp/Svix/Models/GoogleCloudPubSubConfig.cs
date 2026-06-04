// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class GoogleCloudPubSubConfig
    {
        [JsonProperty("credentials", Required = Required.Always)]
        public required string Credentials { get; set; }

        [JsonProperty("projectId", Required = Required.Always)]
        public required string ProjectId { get; set; }

        [JsonProperty("topicId", Required = Required.Always)]
        public required string TopicId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class GoogleCloudPubSubConfig {\n");
            sb.Append("  Credentials: ").Append(Credentials).Append('\n');
            sb.Append("  ProjectId: ").Append(ProjectId).Append('\n');
            sb.Append("  TopicId: ").Append(TopicId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
