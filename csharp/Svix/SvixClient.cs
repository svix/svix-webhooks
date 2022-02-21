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
        protected Configuration Config => new Configuration
        {
            BasePath = ServerUrl,
            AccessToken = Token
        };
        
        public IApplication Application { get; }
        
        public IHealth Health { get; }

        public ILogger Logger { get; }

        public string ServerUrl => _options?.ServerUrl;

        public bool Throw => _options?.Throw ?? false;
        
        public string Token { get; }

        private readonly ISvixOptions _options;

        public SvixClient(string token, ISvixOptions options, ILogger<SvixClient> logger = null, IHealthApi healthApi = null
            , IApplicationApi applicationApi = null)
        {
            Logger = logger;
            _options = options ?? throw new ArgumentNullException(nameof(options));
            Token = token ?? throw new ArgumentNullException(nameof(token));
            
            Health = new Health(this, healthApi ?? new HealthApi(Config));
            Application = new Application(this, applicationApi ?? new ApplicationApi(Config));
        }
        
        public SvixClient(string token, ISvixOptions options, ILogger<SvixClient> logger = null)
            : this(token, options, logger, healthApi: null, applicationApi: null)
        {
            // empty
        }
    }
}