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
        private const string AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_";
        private const string UnsupportedTokenVersion =
            "Unsupported token version. You might need to update the Svix SDK to use this token";

        private readonly string appId;
        private readonly string? endpointId;
        private readonly string? autoconfigId;
        private readonly EndpointIn endpoint;
        private readonly Webhook webhook;
        private readonly SvixClient client;

        public AutoConfig(string token, EndpointIn endpoint)
        {
            endpoint = endpoint ?? throw new ArgumentNullException(nameof(endpoint));

            var content = DecodeAutoConfigToken(token);

            Webhook webhook;
            try
            {
                webhook = new Webhook(content.EndpointSecret);
            }
            catch (Exception e)
            {
                throw new AutoConfigException("invalid token", e);
            }

            appId = content.AppId;
            endpointId = content.EndpointId;
            autoconfigId = content.AutoconfigId;
            this.endpoint = endpoint;
            this.webhook = webhook;
            client = new SvixClient(
                content.TokenPlaintext,
                new SvixOptions(serverUrl: content.ServerUrl)
            );
        }

        public async Task<EndpointOut> SubscribeAsync(CancellationToken cancellationToken = default)
        {
            if (autoconfigId != null)
            {
                return await new EndpointAutoconfig(client).SubscribeAsync(
                    appId,
                    autoconfigId,
                    endpoint,
                    cancellationToken
                );
            }
            return await new EndpointAutoConfigDeprecated(client).UpdateAsync(
                appId,
                endpointId!,
                new SubscribeIn { Endpoint = endpoint },
                cancellationToken
            );
        }

        public EndpointOut Subscribe()
        {
            if (autoconfigId != null)
            {
                return new EndpointAutoconfig(client).Subscribe(appId, autoconfigId, endpoint);
            }
            return new EndpointAutoConfigDeprecated(client).Update(
                appId,
                endpointId!,
                new SubscribeIn { Endpoint = endpoint }
            );
        }

        public void Verify(ReadOnlySpan<char> payload, WebHeaderCollection headers)
        {
            webhook.Verify(payload, headers);
        }

        public void Verify(ReadOnlySpan<char> payload, Func<string?, string?> headersProvider)
        {
            webhook.Verify(payload, headersProvider);
        }

        internal sealed class AutoConfigTokenContentV1
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

        internal sealed class AutoConfigTokenContentV2
        {
            [JsonProperty("aid", Required = Required.Always)]
            public required string AppId { get; set; }

            [JsonProperty("sid", Required = Required.Always)]
            public required string AutoconfigId { get; set; }

            [JsonProperty("surl", Required = Required.Always)]
            public required string ServerUrl { get; set; }

            [JsonProperty("esec", Required = Required.Always)]
            public required string EndpointSecret { get; set; }

            [JsonProperty("tok", Required = Required.Always)]
            public required string TokenPlaintext { get; set; }
        }

        internal sealed class DecodedAutoConfigToken
        {
            public required string AppId { get; set; }
            public string? EndpointId { get; set; }
            public string? AutoconfigId { get; set; }
            public required string ServerUrl { get; set; }
            public required string EndpointSecret { get; set; }
            public required string TokenPlaintext { get; set; }
        }

        private static string DecodeTokenPayload(string token, string prefix)
        {
            token = token ?? throw new ArgumentNullException(nameof(token));

            if (!token.StartsWith(prefix, StringComparison.Ordinal))
            {
                throw new AutoConfigException(UnsupportedTokenVersion);
            }

            var b64 = token.Substring(prefix.Length);

            byte[] decoded;
            try
            {
                decoded = Convert.FromBase64String(b64);
            }
            catch (FormatException e)
            {
                throw new AutoConfigException("invalid token", e);
            }

            return Webhook.SafeUTF8Encoding.GetString(decoded);
        }

        internal static AutoConfigTokenContentV1 DecodeAutoConfigTokenV1(string token)
        {
            try
            {
                var json = DecodeTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V1);
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

        internal static AutoConfigTokenContentV2 DecodeAutoConfigTokenV2(string token)
        {
            try
            {
                var json = DecodeTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V2);
                var content = JsonConvert.DeserializeObject<AutoConfigTokenContentV2>(json);
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

        internal static DecodedAutoConfigToken DecodeAutoConfigToken(string token)
        {
            token = token ?? throw new ArgumentNullException(nameof(token));

            if (token.StartsWith(AUTOCONFIG_TOKEN_PREFIX_V1, StringComparison.Ordinal))
            {
                var content = DecodeAutoConfigTokenV1(token);
                return new DecodedAutoConfigToken
                {
                    AppId = content.AppId,
                    EndpointId = content.EndpointId,
                    ServerUrl = content.ServerUrl,
                    EndpointSecret = content.EndpointSecret,
                    TokenPlaintext = content.TokenPlaintext,
                };
            }
            if (token.StartsWith(AUTOCONFIG_TOKEN_PREFIX_V2, StringComparison.Ordinal))
            {
                var content = DecodeAutoConfigTokenV2(token);
                return new DecodedAutoConfigToken
                {
                    AppId = content.AppId,
                    AutoconfigId = content.AutoconfigId,
                    ServerUrl = content.ServerUrl,
                    EndpointSecret = content.EndpointSecret,
                    TokenPlaintext = content.TokenPlaintext,
                };
            }
            throw new AutoConfigException(UnsupportedTokenVersion);
        }
    }
}
