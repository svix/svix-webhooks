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
            SvixHttpClient? svixHttpClient = null
        )
        {
            Options = options;
            Logger = logger;
            SvixHttpClient =
                svixHttpClient
                ?? new SvixHttpClient(token, options, $"svix-libs/{Version.version}/csharp");
            Application = new Application(this);
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
    }
}
