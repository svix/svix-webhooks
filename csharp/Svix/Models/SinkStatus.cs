// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum SinkStatus
    {
        [EnumMember(Value = "enabled")]
        Enabled = 1,

        [EnumMember(Value = "paused")]
        Paused = 2,

        [EnumMember(Value = "disabled")]
        Disabled = 3,

        [EnumMember(Value = "retrying")]
        Retrying = 4,
    }
}
