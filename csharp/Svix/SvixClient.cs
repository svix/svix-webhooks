using System;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Models;

namespace Svix
{
    public sealed class SvixClient : ISvixClient
    {
        protected Configuration Config =>
            new Configuration { BasePath = ServerUrl, AccessToken = Token };

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

        public ILogger Logger { get; }

        public string ServerUrl => _options?.ServerUrl;

        public bool Throw => _options?.Throw ?? false;

        public string Token { get; }

        private readonly ISvixOptions _options;

        public SvixClient(
            string token,
            ISvixOptions options,
            ILogger<SvixClient> logger = null,
            ApplicationApi applicationApi = null,
            AuthenticationApi authenticationApi = null,
            EndpointApi endpointApi = null,
            EventTypeApi eventTypeApi = null,
            HealthApi healthApi = null,
            IntegrationApi integrationApi = null,
            MessageApi messageApi = null,
            MessageAttemptApi messageAttemptApi = null,
            StatisticsApi statisticsApi = null,
            WebhookEndpointApi operationalWebhookEndpointApi = null
        )
        {
            Logger = logger;
            _options = options ?? throw new ArgumentNullException(nameof(options));
            Token = token ?? throw new ArgumentNullException(nameof(token));

            Application = new Application(this, applicationApi ?? new ApplicationApi(Config));
            Authentication = new Authentication(
                this,
                authenticationApi ?? new AuthenticationApi(Config)
            );
            Endpoint = new Endpoint(this, endpointApi ?? new EndpointApi(Config));
            EventType = new EventType(this, eventTypeApi ?? new EventTypeApi(Config));
            Health = new Health(this, healthApi ?? new HealthApi(Config));
            Integration = new Integration(this, integrationApi ?? new IntegrationApi(Config));
            Message = new Message(this, messageApi ?? new MessageApi(Config));
            MessageAttempt = new MessageAttempt(
                this,
                messageAttemptApi ?? new MessageAttemptApi(Config)
            );
            Statistics = new Statistics(this, statisticsApi ?? new StatisticsApi(Config));
            OperationalWebhookEndpoint = new OperationalWebhookEndpoint(
                this,
                operationalWebhookEndpointApi ?? new WebhookEndpointApi(Config)
            );
        }

        public SvixClient(string token, ISvixOptions options, ILogger<SvixClient> logger)
            : this(token, options, logger, healthApi: null, applicationApi: null)
        {
            // empty
        }
    }
}
