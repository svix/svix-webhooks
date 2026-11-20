// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointOauthConfigIn
    {
        [JsonProperty("grantType", Required = Required.Always)]
        public required Oauth2GrantTypeInOut GrantType { get; set; }

        [JsonProperty("authMethod", Required = Required.Always)]
        public required Oauth2AuthMethodInOut AuthMethod { get; set; }

        [JsonProperty("jwtParams")]
        public ClientSecretJwtParamsIn? JwtParams { get; set; } = null;

        [JsonProperty("clientId", Required = Required.Always)]
        public required string ClientId { get; set; }

        [JsonProperty("clientSecret")]
        public string? ClientSecret { get; set; } = null;

        [JsonProperty("tokenUrl", Required = Required.Always)]
        public required string TokenUrl { get; set; }

        [JsonProperty("scopes")]
        public List<string>? Scopes { get; set; } = null;

        [JsonProperty("extraParams")]
        public Dictionary<string, string>? ExtraParams { get; set; } = null;

        [JsonProperty("refreshToken")]
        public string? RefreshToken { get; set; } = null;

        [JsonProperty("extraHeaders")]
        public Dictionary<string, string>? ExtraHeaders { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointOauthConfigIn {\n");
            sb.Append("  GrantType: ").Append(GrantType).Append('\n');
            sb.Append("  AuthMethod: ").Append(AuthMethod).Append('\n');
            sb.Append("  JwtParams: ").Append(JwtParams).Append('\n');
            sb.Append("  ClientId: ").Append(ClientId).Append('\n');
            sb.Append("  ClientSecret: ").Append(ClientSecret).Append('\n');
            sb.Append("  TokenUrl: ").Append(TokenUrl).Append('\n');
            sb.Append("  Scopes: ").Append(Scopes).Append('\n');
            sb.Append("  ExtraParams: ").Append(ExtraParams).Append('\n');
            sb.Append("  RefreshToken: ").Append(RefreshToken).Append('\n');
            sb.Append("  ExtraHeaders: ").Append(ExtraHeaders).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
