// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    /// <summary>
    /// Defines the ordering in a listing of results.
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum Ordering
    {
        [EnumMember(Value = "ascending")]
        Ascending = 1,

        [EnumMember(Value = "descending")]
        Descending = 2,
    }
}
