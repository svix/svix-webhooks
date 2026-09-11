// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.Api
{
    public class OperationalWebhook(SvixClient client)
    {
        readonly SvixClient _client = client;

        public OperationalWebhookEndpoint Endpoint
        {
            get => new OperationalWebhookEndpoint(_client);
        }
    }
}
