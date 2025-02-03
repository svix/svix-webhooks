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

        public string BaseUrl;
        public string Token;
        public SvixHttpClient SvixHttpClient;

        public SvixClient(
            string baseUrl,
            string token,
            SvixHttpClient? svixHttpClient = null,
            ILogger<SvixClient>? logger = null
        )
        {
            Logger = logger;
            BaseUrl = baseUrl;
            Token = token;
            SvixHttpClient = svixHttpClient ?? new SvixHttpClient(baseUrl, token);
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
