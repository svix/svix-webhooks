using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace Svix.Abstractions
{
    public interface ISvixClient
    {
        public IApplication Application { get; }

        public IAuthentication Authentication { get; }

        public IEndpoint Endpoint { get; }

        public IEventType EventType { get; }

        public IIntegration Integration { get; }

        public IMessage Message { get; }

        public IMessageAttempt MessageAttempt { get; }

        public IHealth Health { get; }

        public ILogger Logger { get; }

        public bool Throw { get; }
    }
}
