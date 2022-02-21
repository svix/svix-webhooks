using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace Svix.Abstractions
{
    public interface ISvixClient
    {
        public IApplication Application { get; }
        
        public IHealth Health { get; }
        
        public ILogger Logger { get; }
        
        public bool Throw { get; }
    }
}