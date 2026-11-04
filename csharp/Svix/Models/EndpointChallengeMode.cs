// this file is @generated
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Svix.Models
{
    /// <summary>
    /// How an endpoint must respond to a challenge.
    ///
    /// - `challenge-response`: the Standard Webhooks challenge-response check.
    ///   The endpoint must return `webhook-challenge-response`, calculated from
    ///   the challenge and its secret.
    /// - `echo`: the endpoint must echo the challenge from the request body back
    ///   as `{"challenge": "..."}`. This doesn't prove that it holds the secret.
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum EndpointChallengeMode
    {
        [EnumMember(Value = "challenge-response")]
        ChallengeResponse = 1,

        [EnumMember(Value = "echo")]
        Echo = 2,
    }
}
