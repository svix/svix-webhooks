using Microsoft.Extensions.Logging;

namespace Svix
{
    public class SvixClient
    {
        public Application Application
        {
            get => new Application(this);
        }

        public Authentication Authentication
        {
            get => new Authentication(this);
        }

        public Endpoint Endpoint
        {
            get => new Endpoint(this);
        }

        public EventType EventType
        {
            get => new EventType(this);
        }

        public Health Health
        {
            get => new Health(this);
        }

        public Ingest Ingest
        {
            get => new Ingest(this);
        }

        public Integration Integration
        {
            get => new Integration(this);
        }

        public Message Message
        {
            get => new Message(this);
        }

        public MessageAttempt MessageAttempt
        {
            get => new MessageAttempt(this);
        }

        public Statistics Statistics
        {
            get => new Statistics(this);
        }

        public Stream Stream
        {
            get => new Stream(this);
        }

        public OperationalWebhook OperationalWebhook
        {
            get => new OperationalWebhook(this);
        }

        public OperationalWebhookEndpoint OperationalWebhookEndpoint
        {
            get => new OperationalWebhookEndpoint(this);
        }

        public ILogger? Logger { get; }

        public SvixHttpClient SvixHttpClient;

        public SvixClient(
            string token,
            SvixOptions? options = null,
            ILogger<SvixClient>? logger = null,
            SvixHttpClient? svixHttpClient = null
        )
        {
            Logger = logger;
            var opts = options ?? new SvixOptions(Utils.DefaultServerUrlFromToken(token));
            SvixHttpClient =
                svixHttpClient
                ?? new SvixHttpClient(
                    token,
                    opts.RetryScheduleMilliseconds,
                    $"svix-libs/{Version.version}/csharp",
                    opts.ServerUrl ?? Utils.DEFAULT_SERVER_URL
                );
        }
    }
}
