// this file is @generated
using System.Runtime.Serialization;
using System.Text.Json.Serialization;

namespace Svix.Models
{
    [JsonConverter(typeof(JsonEnumMemberStringEnumConverter))]
    public enum BackgroundTaskStatus
    {
        [EnumMember(Value = "running")]
        Running = 1,

        [EnumMember(Value = "finished")]
        Finished = 2,

        [EnumMember(Value = "failed")]
        Failed = 3,
    }
}
