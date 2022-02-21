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
        
        public IHealth Health { get; }

        public ILogger Logger { get; }

        public string ServerUrl => _options?.ServerUrl;

        public bool Throw => _options?.Throw ?? false;
        
        public string Token { get; }

        private readonly ISvixOptions _options;

        public SvixClient(string token, SvixOptions options, ILogger<SvixClient> logger = null)
        {
            Logger = logger;
            _options = options ?? throw new ArgumentNullException(nameof(options));
            Token = token ?? throw new ArgumentNullException(nameof(token));
            
            Health = new Health(this, healthApi ?? new HealthApi(Config));
        }
    }
}