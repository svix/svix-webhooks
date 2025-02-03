// this file is @generated
using System.Runtime.Serialization;
using System.Text.Json.Serialization;

namespace Svix.Models
{
    /// <summary>
    /// Defines the ordering in a listing of results.
    /// </summary>
    [JsonConverter(typeof(JsonEnumMemberStringEnumConverter))]
    public enum Ordering
    {
        [EnumMember(Value = "ascending")]
        Ascending = 1,

        [EnumMember(Value = "descending")]
        Descending = 2,
    }
}
