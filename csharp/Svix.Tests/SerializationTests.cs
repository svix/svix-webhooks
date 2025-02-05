using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Svix.Models;
using WireMock.RequestBuilders;
using WireMock.ResponseBuilders;
using WireMock.Server;
using WireMock.Settings;
using Xunit;

namespace Svix.Tests
{
    public class SerializationTests : IDisposable
    {
        private readonly WireMockServer stub;
        private readonly string baseUrl;
        private static readonly string applicationOutJsonStr =
            """{"name":"Test name","id":"app_2raC7cFHmm6rLPcBjbVgeGQOnzr","createdAt":"2025-01-13T17:00:32.241022Z","updatedAt":"2025-02-04T20:50:59.911308Z","metadata":{}}""";
        private SvixClient client;

        public SerializationTests()
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
        public void TestListQueryParamsSerialization()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app/random_app/msg"))
                .RespondWith(
                    Response
                        .Create()
                        .WithStatusCode(200)
                        .WithBodyAsJson(
                            new ListResponseMessageOut
                            {
                                Done = false,
                                Data = new List<MessageOut> { },
                            }
                        )
                );

            var svx = new SvixClient("", new SvixOptions(baseUrl));
            svx.Message.List(
                "random_app",
                new MessageListOptions
                {
                    EventTypes = new List<string> { "val1", "val2", "val3" },
                }
            );
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.Equal(
                "?event_types=val1%2Cval2%2Cval3",
                stub.LogEntries[0].RequestMessage.RawQuery
            );
        }

        [Fact]
        public void NonCamelCaseIsCorrectlySerialized()
        {
            var a = new EventTypeImportOpenApiOutData
            {
                ToModify = new List<EventTypeFromOpenApi> { },
                Modified = new List<string> { },
            };

            string encoded_json = JsonConvert.SerializeObject(a);
            string expected_json = """"{"modified":[],"to_modify":[]}"""";
            Assert.Equal(expected_json, encoded_json);
        }

        [Fact]
        public void StringEnumCorrectlySerialized()
        {
            var a = BackgroundTaskType.ApplicationStats;
            string encoded_json = JsonConvert.SerializeObject(a);
            // this enum gets encoded as a single value json object, so it has extra quotes
            string expected_json = "\"application.stats\"";
            Assert.Equal(expected_json, encoded_json);
        }

        [Fact]
        public void StringEnumCorrectlyDeserialized()
        {
            string raw_json = """"{"id":"asd","status":"finished","task":"application.stats"}"""";
            var loaded_from_str = JsonConvert.DeserializeObject<ReplayOut>(raw_json);
            var expected = new ReplayOut
            {
                Id = "asd",
                Status = BackgroundTaskStatus.Finished,
                Task = BackgroundTaskType.ApplicationStats,
            };
            Assert.Equal(expected.Id, loaded_from_str.Id);
            Assert.Equal(expected.Status, loaded_from_str.Status);
            Assert.Equal(expected.Task, loaded_from_str.Task);
        }

        [Fact]
        public void ArbitraryJsonObjectCorrectlySerialized()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            client.Application.Create(
                new ApplicationIn
                {
                    Name = "app",
                    Metadata = new Dictionary<string, string>
                    {
                        { "key1", "val1" },
                        { "key2", "val2" },
                    },
                }
            );

            string expected_json_body =
                """{"metadata":{"key1":"val1","key2":"val2"},"name":"app"}""";
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.Equal(expected_json_body, stub.LogEntries[0].RequestMessage.Body);
        }

        [Fact]
        public void PatchRequestBodyIgnoresUnsetFields()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app/*"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            client.Application.Patch("app1", new ApplicationPatch { Name = "app" });

            string expected_json_body = """{"name":"app"}""";
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.Equal(expected_json_body, stub.LogEntries[0].RequestMessage.Body);
        }

        [Fact]
        public void PatchRequestBodySerializesExplicitlyNullFields()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app/*"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            client.Application.Patch("app1", new ApplicationPatch { RateLimit = null });

            string expected_json_body = """{"rateLimit":null}""";
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.Equal(expected_json_body, stub.LogEntries[0].RequestMessage.Body);
        }

        [Fact]
        public void NonePatchRequestBodyIgnoresNullFields()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            client.Application.Create(new ApplicationIn { Name = "app1" });

            string expected_json_body = """{"name":"app1"}""";
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.Equal(expected_json_body, stub.LogEntries[0].RequestMessage.Body);
        }
    }
}
