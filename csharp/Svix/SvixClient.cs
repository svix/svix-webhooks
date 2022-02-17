using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Models;
using System;
using System.Net;
using System.Threading;
using System.Threading.Tasks;

namespace Svix
{
    public sealed class SvixClient : ISvixClient
    {
        public Health Health { get; init; }
        
        private readonly ILogger _logger;
        
        private readonly SvixClientOptions _options;

        public SvixClient(SvixClientOptions options, ILogger<SvixClient> logger = null)
        {
            _options = options ?? throw new ArgumentNullException(nameof(options));
            _logger = logger;

            Health = new Health(options, logger);
        }

        public SvixClient(IOptions<SvixClientOptions> options, ILogger<SvixClient> logger)
            : this(options?.Value, logger)
        {
            // empty
        }
    }
}