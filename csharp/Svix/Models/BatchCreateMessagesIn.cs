// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class BatchCreateMessagesIn
    {
        [JsonProperty("messages", Required = Required.Always)]
        public required List<BatchMessageIn> Messages { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BatchCreateMessagesIn {\n");
            sb.Append("  Messages: ").Append(Messages).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
