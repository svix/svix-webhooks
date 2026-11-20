// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointOauthConfigOut
    {
        [JsonProperty("grantType", Required = Required.Always)]
        public required Oauth2GrantTypeInOut GrantType { get; set; }

        [JsonProperty("authMethod", Required = Required.Always)]
        public required Oauth2AuthMethodInOut AuthMethod { get; set; }

        [JsonProperty("clientId", Required = Required.Always)]
        public required string ClientId { get; set; }

        [JsonProperty("tokenUrl", Required = Required.Always)]
        public required string TokenUrl { get; set; }

        [JsonProperty("scopes")]
        public List<string>? Scopes { get; set; } = null;

        [JsonProperty("extraParams")]
        public Dictionary<string, string>? ExtraParams { get; set; } = null;

        [JsonProperty("extraHeaders")]
        public Dictionary<string, string>? ExtraHeaders { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointOauthConfigOut {\n");
            sb.Append("  GrantType: ").Append(GrantType).Append('\n');
            sb.Append("  AuthMethod: ").Append(AuthMethod).Append('\n');
            sb.Append("  ClientId: ").Append(ClientId).Append('\n');
            sb.Append("  TokenUrl: ").Append(TokenUrl).Append('\n');
            sb.Append("  Scopes: ").Append(Scopes).Append('\n');
            sb.Append("  ExtraParams: ").Append(ExtraParams).Append('\n');
            sb.Append("  ExtraHeaders: ").Append(ExtraHeaders).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
