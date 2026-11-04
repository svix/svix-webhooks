// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum ArchiveStatus
    {
        [EnumMember(Value = "active")]
        Active = 1,

        [EnumMember(Value = "archived")]
        Archived = 2,

        [EnumMember(Value = "all")]
        All = 3,
    }
}
