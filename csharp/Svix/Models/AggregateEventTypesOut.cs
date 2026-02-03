// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AggregateEventTypesOut
    {
        [JsonProperty("id", Required = Required.Always)]
        public required string Id { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required BackgroundTaskStatus Status { get; set; }

        [JsonProperty("task", Required = Required.Always)]
        public required BackgroundTaskType Task { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public required DateTime UpdatedAt { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AggregateEventTypesOut {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Task: ").Append(Task).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
