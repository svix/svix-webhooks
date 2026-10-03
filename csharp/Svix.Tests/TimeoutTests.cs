using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using Svix.Models;
using WireMock.RequestBuilders;
using WireMock.ResponseBuilders;
using WireMock.Server;
using Xunit;

namespace Svix.Tests
{
    public class TimeoutTests : IDisposable
    {
        private readonly WireMockServer stub = WireMockServer.Start();
        private static readonly string applicationOutJsonStr =
            """{"name":"Test name","id":"app1","createdAt":"2025-01-13T17:00:32.241022Z","updatedAt":"2025-02-04T20:50:59.911308Z","metadata":{}}""";

        public void Dispose() => stub.Dispose();

        private void RespondWithDelay(int delayMilliseconds, int statusCode = 200)
        {
            stub.ResetMappings();
            var response = Response
                .Create()
                .WithStatusCode(statusCode)
                .WithBody(applicationOutJsonStr);
            if (delayMilliseconds > 0)
            {
                response.WithDelay(delayMilliseconds);
            }
            stub.Given(Request.Create().WithPath("/api/v1/app/app1").UsingGet())
                .RespondWith(response);
        }

        private int RequestCount =>
            stub.LogEntries.Count(entry => entry.RequestMessage.Path == "/api/v1/app/app1");

        [Theory]
        [InlineData(false)]
        [InlineData(true)]
        public async Task ConfiguredTimeoutIsAppliedWithoutRetrying(bool sync)
        {
            RespondWithDelay(2000);
            var client = new SvixClient("", new SvixOptions(stub.Url, 500));

            if (sync)
            {
                Assert.ThrowsAny<OperationCanceledException>(() => client.Application.Get("app1"));
            }
            else
            {
                await Assert.ThrowsAnyAsync<OperationCanceledException>(() =>
                    client.Application.GetAsync("app1")
                );
            }

            // WireMock records requests after the delayed response is built.
            await Task.Delay(2200);
            Assert.Equal(1, RequestCount);
        }

        [Theory]
        [InlineData(500, 0)]
        [InlineData(4000, 2000)]
        [InlineData(Timeout.Infinite, 2000)]
        public async Task ResponsesWithinTimeoutSucceed(
            int timeoutMilliseconds,
            int delayMilliseconds
        )
        {
            RespondWithDelay(delayMilliseconds);
            var client = new SvixClient("", new SvixOptions(stub.Url, timeoutMilliseconds));

            var application = await client.Application.GetAsync("app1");

            Assert.Equal("app1", application.Id);
            Assert.Equal(1, RequestCount);
        }

        [Theory]
        [InlineData(false)]
        [InlineData(true)]
        public async Task InjectedTransportKeepsItsTimeout(bool alreadyUsed)
        {
            var transport = new SvixHttpClient("", new List<int>(), "test", stub.Url);
            if (alreadyUsed)
            {
                RespondWithDelay(0);
                await transport.SendRequestAsync<ApplicationOut>(
                    HttpMethod.Get,
                    "/api/v1/app/app1"
                );
                stub.ResetLogEntries();
            }
            RespondWithDelay(2000);
            var client = new SvixClient(
                "",
                new SvixOptions(stub.Url, 500),
                svixHttpClient: transport
            );

            var application = await client.Application.GetAsync("app1");

            Assert.Same(transport, client.SvixHttpClient);
            Assert.Equal("app1", application.Id);
            Assert.Equal(1, RequestCount);
        }

        [Fact]
        public async Task TimeoutAppliesToEachRetryAttempt()
        {
            RespondWithDelay(200, 500);
            var client = new SvixClient(
                "",
                new SvixOptions(stub.Url, 1000, new List<int> { 1200 })
            );

            var error = await Assert.ThrowsAsync<ApiException>(() =>
                client.Application.GetAsync("app1")
            );

            Assert.Equal(500, error.ErrorCode);
            Assert.Equal(2, RequestCount);
        }
    }
}
