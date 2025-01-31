using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace Svix.Abstractions
{
    public interface ISvixClient
    {
        public Authentication Authentication { get; }

        public Endpoint Endpoint { get; }

        public EventType EventType { get; }

        public Integration Integration { get; }

        public Message Message { get; }

        public MessageAttempt MessageAttempt { get; }

        public Health Health { get; }

        public ILogger Logger { get; }

        public bool Throw { get; }
    }
}
