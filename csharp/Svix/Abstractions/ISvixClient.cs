using System.Threading;
using System.Threading.Tasks;

namespace Svix.Abstractions
{
    public interface ISvixClient
    {
        public Health Health { get; }
    }
}