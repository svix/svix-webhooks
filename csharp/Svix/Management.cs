// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class Management(SvixClient client)
    {
        readonly SvixClient _client = client;

        public ManagementAuthentication Authentication
        {
            get => new ManagementAuthentication(_client);
        }
    }
}
