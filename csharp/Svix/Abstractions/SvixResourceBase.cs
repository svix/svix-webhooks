using System;
using Microsoft.Extensions.Logging;

namespace Svix.Abstractions
{
    public abstract class SvixResourceBase
    {
        private SvixResourceBase()
        {
            // empty
        }

        protected ILogger Logger => SvixClient.Logger;

        protected readonly ISvixClient SvixClient;

        protected bool Throw => SvixClient?.Throw ?? false;

        protected SvixResourceBase(ISvixClient svixClient)
        {
            SvixClient = svixClient ?? throw new ArgumentNullException(nameof(svixClient));
        }
    }
}