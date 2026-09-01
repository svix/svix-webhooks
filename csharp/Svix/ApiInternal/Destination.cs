// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class Destination(SvixClient client)
    {
        readonly SvixClient _client = client;

        public DestinationAutoconfig Autoconfig
        {
            get => new DestinationAutoconfig(_client);
        }
    }
}
