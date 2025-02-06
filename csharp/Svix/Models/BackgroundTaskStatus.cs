// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
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
