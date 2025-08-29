// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum MessageStatusText
    {
        [EnumMember(Value = "success")]
        Success = 1,

        [EnumMember(Value = "pending")]
        Pending = 2,

        [EnumMember(Value = "fail")]
        Fail = 3,

        [EnumMember(Value = "sending")]
        Sending = 4,
    }
}
