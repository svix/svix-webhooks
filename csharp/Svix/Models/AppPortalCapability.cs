// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum AppPortalCapability
    {
        [EnumMember(Value = "ViewBase")]
        ViewBase = 1,

        [EnumMember(Value = "ViewEndpointSecret")]
        ViewEndpointSecret = 2,

        [EnumMember(Value = "ManageEndpointSecret")]
        ManageEndpointSecret = 3,

        [EnumMember(Value = "ManageTransformations")]
        ManageTransformations = 4,

        [EnumMember(Value = "CreateAttempts")]
        CreateAttempts = 5,

        [EnumMember(Value = "ManageEndpoint")]
        ManageEndpoint = 6,
    }
}
