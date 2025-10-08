// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum SinkStatusIn
    {
        [EnumMember(Value = "enabled")]
        Enabled = 1,

        [EnumMember(Value = "disabled")]
        Disabled = 2,
    }
}
