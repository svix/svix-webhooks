// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum OauthJwsSigningAlgorithm
    {
        [EnumMember(Value = "RS256")]
        Rs256 = 1,
    }
}
