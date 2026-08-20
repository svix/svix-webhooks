// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum BulkExpungeStatus
    {
        [EnumMember(Value = "expunged")]
        Expunged = 1,

        [EnumMember(Value = "not-found")]
        NotFound = 2,
    }
}
