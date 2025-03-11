// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum EndpointDisabledTrigger
    {
        [EnumMember(Value = "manual")]
        Manual = 1,

        [EnumMember(Value = "automatic")]
        Automatic = 2,
    }
}
