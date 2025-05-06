using System;
using System.Collections.Generic;
using WireMock.RequestBuilders;
using WireMock.ResponseBuilders;
using WireMock.Server;
using WireMock.Settings;
using Xunit;

namespace Svix.Tests
{
    public class RetryScheduleTests : IDisposable
    {
        private readonly WireMockServer stub;
        private SvixClient client;

        public RetryScheduleTests()
        {
            var port = new Random().Next(5000, 6000);
            var baseUrl = "http://localhost:" + port;
            client = new SvixClient("", new SvixOptions(baseUrl));
            stub = WireMockServer.Start(
                new WireMockServerSettings { Urls = new[] { "http://+:" + port } }
            );
        }

        public void Dispose() => Dispose(true);

        protected virtual void Dispose(bool disposing)
        {
            if (disposing)
            {
                stub.Stop();
                stub.Dispose();
            }
        }

        [Fact]
        public void ClientSendsRetryHeaders()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(Response.Create().WithStatusCode(500));
            try
            {
                client.Application.List();
            }
            catch (ApiException) { }

            // 4 requests, first default request and 3 retry requests
            Assert.Equal(4, stub.LogEntries.Count);

            var req_id = stub.LogEntries[0].RequestMessage.Headers["svix-req-id"];
            foreach (var req in stub.LogEntries)
            {
                // check the request id is the same for all retries
                Assert.Equal(req_id, req.RequestMessage.Headers["svix-req-id"]);
            }
            // check the last 3 request have the correct retry header
            for (var index = 0; index < 4; index++)
            {
                if (index == 0)
                {
                    Assert.Throws<KeyNotFoundException>(() =>
                        stub.LogEntries[index].RequestMessage.Headers["svix-retry-count"]
                    );
                    continue;
                }
                var retryCount = stub.LogEntries[index].RequestMessage.Headers["svix-retry-count"];
                Assert.Equal(retryCount, index.ToString());
            }
        }
    }
}
