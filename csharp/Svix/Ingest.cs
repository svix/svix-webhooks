// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class Ingest(SvixClient client)
    {
        readonly SvixClient _client = client;

        public IngestAuthentication Authentication
        {
            get => new IngestAuthentication(_client);
        }

        public IngestEndpoint Endpoint
        {
            get => new IngestEndpoint(_client);
        }

        public IngestSource Source
        {
            get => new IngestSource(_client);
        }
    }
}
