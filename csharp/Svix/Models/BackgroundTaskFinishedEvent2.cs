// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BackgroundTaskFinishedEvent2
    {
        [JsonProperty("data", Required = Required.Always)]
        public required Object Data { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public required BackgroundTaskStatus Status { get; set; }

        [JsonProperty("task", Required = Required.Always)]
        public required BackgroundTaskType Task { get; set; }

        [JsonProperty("taskId", Required = Required.Always)]
        public required string TaskId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BackgroundTaskFinishedEvent2 {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Task: ").Append(Task).Append('\n');
            sb.Append("  TaskId: ").Append(TaskId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
