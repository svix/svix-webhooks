using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;
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
        private readonly string baseUrl;
        private SvixClient client;

        public RetryScheduleTests()
        {
            var port = new Random().Next(5000, 6000);
            baseUrl = "http://localhost:" + port;
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

        [Fact]
        public async Task CancellationDuringRetryDelayAbortsTheCall()
        {
            var retryDelayMs = 5000;
            var cancelledWithinMs = 2000;
            stub.Given(Request.Create().WithPath("/api/v1/app/app1"))
                .RespondWith(Response.Create().WithStatusCode(500));
            var slowRetryClient = new SvixClient(
                "",
                new SvixOptions(baseUrl, retryScheduleMilliseconds: new List<int> { retryDelayMs })
            );
            using var cts = new CancellationTokenSource();

            // Task.Run so that a blocking wait inside the client cannot stall the test thread itself.
            var call = Task.Run(() => slowRetryClient.Application.GetAsync("app1", cts.Token));

            // The first attempt has been answered with a 500, so the client is now in its retry delay.
            Assert.True(SpinWait.SpinUntil(() => stub.LogEntries.Count >= 1, 5000));
            cts.CancelAfter(100);
            var sw = Stopwatch.StartNew();

            var finished = await Task.WhenAny(call, Task.Delay(cancelledWithinMs));

            Assert.True(
                ReferenceEquals(finished, call),
                $"call still running {sw.ElapsedMilliseconds} ms after cancellation, expected it to abort the {retryDelayMs} ms retry delay"
            );
            await Assert.ThrowsAnyAsync<OperationCanceledException>(() => call);
            // The retry must not have been sent after the cancellation.
            Assert.Single(stub.LogEntries);
        }

        [Theory]
        [InlineData(false)]
        [InlineData(true)]
        public async Task EachRetryWaitsAtLeastTheConfiguredDelay(bool sync)
        {
            var schedule = new List<int> { 150, 300 };
            // Task.Delay can fire slightly early, so allow some slack below the configured delay.
            var toleranceMs = 50;
            stub.Given(Request.Create().WithPath("/api/v1/app/app1"))
                .RespondWith(Response.Create().WithStatusCode(500));
            var scheduledClient = new SvixClient(
                "",
                new SvixOptions(baseUrl, retryScheduleMilliseconds: schedule)
            );

            if (sync)
            {
                Assert.Throws<ApiException>(() => scheduledClient.Application.Get("app1"));
            }
            else
            {
                await Assert.ThrowsAsync<ApiException>(() =>
                    scheduledClient.Application.GetAsync("app1")
                );
            }

            // The first attempt plus one retry per schedule entry, and nothing after the schedule is exhausted.
            Assert.Equal(schedule.Count + 1, stub.LogEntries.Count);
            for (var index = 0; index < schedule.Count; index++)
            {
                var gap =
                    stub.LogEntries[index + 1].RequestMessage.DateTime
                    - stub.LogEntries[index].RequestMessage.DateTime;
                Assert.True(
                    gap.TotalMilliseconds >= schedule[index] - toleranceMs,
                    $"retry {index + 1} was sent {gap.TotalMilliseconds} ms after the previous attempt, expected at least {schedule[index] - toleranceMs} ms"
                );
            }
        }
    }
}
