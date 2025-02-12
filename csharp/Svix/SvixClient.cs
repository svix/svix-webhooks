using Microsoft.Extensions.Logging;

namespace Svix
{
    public class SvixClient
    {
        public Application Application { get; }

        public Authentication Authentication { get; }

        public Endpoint Endpoint { get; }

        public EventType EventType { get; }

        public Health Health { get; }

        public Integration Integration { get; }

        public Message Message { get; }

        public MessageAttempt MessageAttempt { get; }

        public Statistics Statistics { get; }

        public OperationalWebhookEndpoint OperationalWebhookEndpoint { get; }

        public ILogger? Logger { get; }

        private readonly SvixOptions Options;
        public SvixHttpClient SvixHttpClient;

        public SvixClient(
            string token,
            SvixOptions options,
            ILogger<SvixClient>? logger = null,
            SvixHttpClient? svixHttpClient = null,
            Application? application = null
        )
        {
            Options = options;
            Logger = logger;
            SvixHttpClient = svixHttpClient ?? new SvixHttpClient(token, options, GetUserAgent());
            Application = application ?? new Application(this);
            Authentication = new Authentication(this);
            Endpoint = new Endpoint(this);
            EventType = new EventType(this);
            Health = new Health(this);
            Integration = new Integration(this);
            Message = new Message(this);
            MessageAttempt = new MessageAttempt(this);
            Statistics = new Statistics(this);
            OperationalWebhookEndpoint = new OperationalWebhookEndpoint(this);
        }

        public string GetUserAgent()
        {
            var versionQuad = GetType().Assembly.GetName().Version;

            if (versionQuad != null)
            {
                string versionQuadStr = versionQuad.ToString();
                string version;
                // C# adds an extra trailing zero so the version looks like this "1.56.0.0"
                // remove trailing zero for consistency with other libs
                if (versionQuadStr.EndsWith(".0") && versionQuadStr.Split('.').Length == 4)
                {
                    // Remove the last ".0"
                    version = versionQuadStr[..^2];
                }
                else
                {
                    version = versionQuadStr;
                }

                return $"svix-libs/{version}/csharp";
            }
            else
            {
                // If for some reason we are unable to access the version, don't panic with a nullptr deref
                return "svix-libs/missing-version/csharp";
            }
        }
    }
}
