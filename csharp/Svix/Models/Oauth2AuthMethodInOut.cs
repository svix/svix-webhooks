// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    /// <summary>
    /// The method used for authenticating to the OAuth authorization server.
    ///
    /// `clientSecretJwt` will construct a JWT used for authentication
    /// with the oauth authorization server. This method is less
    /// commonly used and may not be supported by all oauth providers.
    /// `clientSecretBasic` will authenticate to the oauth authorization server
    /// using an `Authorization` header with the client secret as the value.
    /// This is the most common means of authentication.
    /// `clientSecretPost` will authenticate to the oauth authorization server
    /// by passing the client secret in a `client_secret` field in the
    /// request body. This method may not be supported by all oauth providers,
    /// and in general `clientSecretBasic` should be preferred.
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum Oauth2AuthMethodInOut
    {
        [EnumMember(Value = "clientSecretJwt")]
        ClientSecretJwt = 1,

        [EnumMember(Value = "clientSecretBasic")]
        ClientSecretBasic = 2,

        [EnumMember(Value = "clientSecretPost")]
        ClientSecretPost = 3,
    }
}
