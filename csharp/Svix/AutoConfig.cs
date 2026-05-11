using System.Net;
using Newtonsoft.Json;
using Svix.ApiInternal;
using Svix.Models;

namespace Svix
{
    public class AutoConfigException : Exception
    {
        public AutoConfigException()
            : base("invalid token") { }

        public AutoConfigException(string message)
            : base(message) { }

        public AutoConfigException(string message, Exception inner)
            : base(message, inner) { }
    }

    public class AutoConfig
    {
        private const string AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_";

        private readonly string _appId;
        private readonly string _endpointId;
        private readonly EndpointIn _endpoint;
        private readonly Webhook _webhook;
        private readonly SvixClient _client;

        public AutoConfig(string token, EndpointIn endpoint)
        {
            endpoint = endpoint ?? throw new ArgumentNullException(nameof(endpoint));

            var content = DecodeAutoConfigTokenV1(token);

            Webhook webhook;
            try
            {
                webhook = new Webhook(content.EndpointSecret);
            }
            catch (Exception e)
            {
                throw new AutoConfigException("invalid token", e);
            }

            _appId = content.AppId;
            _endpointId = content.EndpointId;
            _endpoint = endpoint;
            _webhook = webhook;
            _client = new SvixClient(
                content.TokenPlaintext,
                new SvixOptions(serverUrl: content.ServerUrl)
            );
        }

        public async Task<EndpointOut> SubscribeAsync(CancellationToken cancellationToken = default)
        {
            return await new EndpointAutoConfig(_client).UpdateAsync(
                _appId,
                _endpointId,
                new SubscribeIn { Endpoint = _endpoint },
                cancellationToken
            );
        }

        public EndpointOut Subscribe()
        {
            return new EndpointAutoConfig(_client).Update(
                _appId,
                _endpointId,
                new SubscribeIn { Endpoint = _endpoint }
            );
        }

        public void Verify(ReadOnlySpan<char> payload, WebHeaderCollection headers)
        {
            _webhook.Verify(payload, headers);
        }

        public void Verify(ReadOnlySpan<char> payload, Func<string?, string?> headersProvider)
        {
            _webhook.Verify(payload, headersProvider);
        }

        private sealed class AutoConfigTokenContentV1
        {
            [JsonProperty("aid", Required = Required.Always)]
            public required string AppId { get; set; }

            [JsonProperty("eid", Required = Required.Always)]
            public required string EndpointId { get; set; }

            [JsonProperty("surl", Required = Required.Always)]
            public required string ServerUrl { get; set; }

            [JsonProperty("esec", Required = Required.Always)]
            public required string EndpointSecret { get; set; }

            [JsonProperty("tok", Required = Required.Always)]
            public required string TokenPlaintext { get; set; }
        }

        private static AutoConfigTokenContentV1 DecodeAutoConfigTokenV1(string token)
        {
            token = token ?? throw new ArgumentNullException(nameof(token));

            if (!token.StartsWith(AUTOCONFIG_TOKEN_PREFIX_V1, StringComparison.Ordinal))
            {
                throw new AutoConfigException();
            }

            var b64 = token.Substring(AUTOCONFIG_TOKEN_PREFIX_V1.Length);

            byte[] decoded;
            try
            {
                decoded = Convert.FromBase64String(b64);
            }
            catch (FormatException e)
            {
                throw new AutoConfigException("invalid token", e);
            }

            try
            {
                var json = Webhook.SafeUTF8Encoding.GetString(decoded);
                var content = JsonConvert.DeserializeObject<AutoConfigTokenContentV1>(json);
                if (content == null)
                {
                    throw new AutoConfigException();
                }
                return content;
            }
            catch (AutoConfigException)
            {
                throw;
            }
            catch (Exception e)
            {
                throw new AutoConfigException("invalid token", e);
            }
        }
    }
}
