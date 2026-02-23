// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessagePrecheckOut
    {
        [JsonProperty("active", Required = Required.Always)]
        public required bool Active { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessagePrecheckOut {\n");
            sb.Append("  Active: ").Append(Active).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
