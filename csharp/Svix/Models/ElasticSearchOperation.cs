// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    /// <summary>
    /// Operation for bulk indexing.
    ///
    /// `index` will do an upsert for the given document ID, and `create` will insert it
    /// (and do nothing on conflict). Only `create` is supported for Data Stream targets.
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum ElasticSearchOperation
    {
        [EnumMember(Value = "index")]
        Index = 1,

        [EnumMember(Value = "create")]
        Create = 2,
    }
}
