// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum ConnectorProduct
    {
        [EnumMember(Value = "Dispatch")]
        Dispatch = 1,

        [EnumMember(Value = "Stream")]
        Stream = 2,
    }
}
