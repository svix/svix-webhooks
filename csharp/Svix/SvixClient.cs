using System;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Models;

namespace Svix
{
    public sealed class SvixClient : ISvixClient
    {
        public Health Health { get; }
        
        public ILogger Logger { get; }

        public string ServerUrl => _options?.ServerUrl;

        public bool Throw => _options?.Throw ?? false;
        
        public string Token { get; }

        private readonly SvixOptions _options;

        public SvixClient(string token, SvixOptions options, ILogger<SvixClient> logger = null)
        {
            Logger = logger;
            _options = options ?? throw new ArgumentNullException(nameof(options));
            Token = token ?? throw new ArgumentNullException(nameof(token));
            
            Health = new Health(this);
        }
    }
}