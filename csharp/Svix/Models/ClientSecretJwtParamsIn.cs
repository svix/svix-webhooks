// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ClientSecretJwtParamsIn
    {
        [JsonProperty("secretBase64", Required = Required.Always)]
        public required string SecretBase64 { get; set; }

        [JsonProperty("signingAlgorithm", Required = Required.Always)]
        public required OauthJwsSigningAlgorithm SigningAlgorithm { get; set; }

        [JsonProperty("secretId")]
        public string? SecretId { get; set; } = null;

        [JsonProperty("tokenExpirySecs")]
        public ulong? TokenExpirySecs { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ClientSecretJwtParamsIn {\n");
            sb.Append("  SecretBase64: ").Append(SecretBase64).Append('\n');
            sb.Append("  SigningAlgorithm: ").Append(SigningAlgorithm).Append('\n');
            sb.Append("  SecretId: ").Append(SecretId).Append('\n');
            sb.Append("  TokenExpirySecs: ").Append(TokenExpirySecs).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
