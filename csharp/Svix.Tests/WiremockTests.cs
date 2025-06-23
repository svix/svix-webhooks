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
        public void UserAgentIsSent()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app/*"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            client.Application.Patch("app1", new ApplicationPatch { Name = "app" });

            Assert.Equal(1, stub.LogEntries.Count);
            Assert.True(stub.LogEntries[0].RequestMessage.Headers.ContainsKey("User-Agent"));
            Assert.StartsWith(
                "svix-libs/",
                stub.LogEntries[0].RequestMessage.Headers["User-Agent"][0]
            );
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

        [Fact]
        public void MessageInRawIsSerializedCorrectly()
        {
            // just checking that the correct body is sent to the server, response is discarded

            stub.Given(Request.Create().WithPath("/api/v1/app/app_asd123/msg"))
                .RespondWith(
                    Response
                        .Create()
                        .WithStatusCode(200)
                        .WithBody(
                            """{"id":"msg_asd13","eventType":"event.type","payload":{},"timestamp":"2025-01-13T17:00:32.241022Z"}"""
                        )
                );

            var msg = Message.messageInRaw("event.type", "not json", "nonstandard/content_type");
            client.Message.Create("app_asd123", msg);

            string expected_json_body = """
                {"eventType":"event.type","payload":{},"transformationsParams":{"rawPayload":"not json","headers":{"content-type":"nonstandard/content_type"}}}
                """;
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.Equal(expected_json_body, stub.LogEntries[0].RequestMessage.Body);
        }

        [Fact]
        public void ApplicationCreate_WithoutApplication_ThrowsException()
        {
            // Don't need to create a stub since the ArgumentNullException will be raised before we try to use the server
            Assert.Throws<ArgumentNullException>(() => client.Application.Create(null, null));
        }

        [Fact]
        public async void ApplicationCreateAsync_WithoutApplication_ThrowsException()
        {
            // Don't need to create a stub since the ArgumentNullException will be raised before we try to use the server
            await Assert.ThrowsAsync<ArgumentNullException>(() =>
                client.Application.CreateAsync(null, null, default)
            );
        }

        [Fact]
        public void ListResponseApplicationOutWorksCorrectly()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(
                    Response
                        .Create()
                        .WithStatusCode(200)
                        .WithBody("""{"data":[],"iterator":null,"prevIterator":null,"done":true}""")
                );
            client.Application.List();
        }

        [Fact]
        public void UrlEncodedOctothorpe()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app/app_id/msg"))
                .RespondWith(
                    Response
                        .Create()
                        .WithStatusCode(200)
                        .WithBody("""{"data":[],"iterator":null,"prevIterator":null,"done":true}""")
                );
            client.Message.List("app_id", new MessageListOptions { Tag = "test#test" });
            Assert.Equal(1, stub.LogEntries.Count);
            Assert.EndsWith(
                "/api/v1/app/app_id/msg?tag=test%23test",
                stub.LogEntries[0].RequestMessage.Url
            );
        }

        [Fact]
        public void StructEnumWithFieldsSerializesCorrectly()
        {
            var jsonString =
                """{"name":"me","uid":"test","type":"cron","config":{"payload":"asd","schedule":"asd"}}""";
            var sourceIn = new IngestSourceIn
            {
                Name = "me",
                Uid = "test",
                Config = IngestSourceInConfig.Cron(
                    new CronConfig { Payload = "asd", Schedule = "asd" }
                ),
            };

            var loadedFromJson = JsonConvert.DeserializeObject<IngestSourceIn>(jsonString);
            Assert.Equal(sourceIn.Name, loadedFromJson.Name);
            Assert.Equal(sourceIn.Uid, loadedFromJson.Uid);

            var loadedFromJsonConfig = (CronConfig)loadedFromJson.Config.GetContent();
            var sourceInConfig = (CronConfig)sourceIn.Config.GetContent();
            Assert.Equal(loadedFromJsonConfig.ContentType, sourceInConfig.ContentType);
            Assert.Equal(loadedFromJsonConfig.Schedule, sourceInConfig.Schedule);
            Assert.Equal(loadedFromJsonConfig.Payload, sourceInConfig.Payload);
        }

        [Fact]
        public void StructEnumWithoutFieldsSerializesCorrectly()
        {
            var jsonString = """{"name":"me","uid":"test","type":"generic-webhook","config":{}}""";
            var sourceIn = new IngestSourceIn
            {
                Name = "me",
                Uid = "test",
                Config = IngestSourceInConfig.GenericWebhook(),
            };

            var loadedFromJson = JsonConvert.DeserializeObject<IngestSourceIn>(jsonString);
            Assert.Equal(sourceIn.Name, loadedFromJson.Name);
            Assert.Equal(sourceIn.Uid, loadedFromJson.Uid);
            Assert.Equal(sourceIn.Config.GetContent(), loadedFromJson.Config.GetContent());
        }

        [Fact]
        public void ReadStructEnumField()
        {
            var jsonString =
                """{"name":"me","uid":"test","type":"cron","config":{"payload":"asd","schedule":"* * * * *"}}""";
            var loadedFromJson = JsonConvert.DeserializeObject<IngestSourceIn>(jsonString);
            Assert.True(loadedFromJson.Config.GetContent().GetType() == typeof(CronConfig));
            Assert.Equal("asd", ((CronConfig)loadedFromJson.Config.GetContent()).Payload);
            Assert.Equal("* * * * *", ((CronConfig)loadedFromJson.Config.GetContent()).Schedule);
        }

        [Fact]
        public void OpWebhookModels()
        {
            var jsonString =
                """{"data":{"data":{"appStats":[{"appId":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","appUid":null,"messageDestinations":343}]},"status":"finished","task":"application.stats","taskId":"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2"},"type":"background_task.finished"}""";
            var loadedFromJson = JsonConvert.DeserializeObject<BackgroundTaskFinishedEvent>(
                jsonString
            );
            Assert.Equal(jsonString, JsonConvert.SerializeObject(loadedFromJson));
        }

        [Fact]
        public void IdempotencyKeyIsSentForListRequest()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(
                    Response
                        .Create()
                        .WithStatusCode(200)
                        .WithBody("""{"data":[],"iterator":null,"prevIterator":null,"done":true}""")
                );
            client.Application.List();

            Assert.Equal(1, stub.LogEntries.Count);
            Assert.True(stub.LogEntries[0].RequestMessage.Headers.ContainsKey("idempotency-key"));
            Assert.StartsWith(
                "auto_",
                stub.LogEntries[0].RequestMessage.Headers["idempotency-key"][0]
            );
            Assert.Equal(1, stub.LogEntries.Count);
        }

        [Fact]
        public void IdempotencyKeyIsSentForCreateRequest()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            client.Application.Create(new ApplicationIn { Name = "app" });

            Assert.Equal(1, stub.LogEntries.Count);
            Assert.True(stub.LogEntries[0].RequestMessage.Headers.ContainsKey("idempotency-key"));
            Assert.StartsWith(
                "auto_",
                stub.LogEntries[0].RequestMessage.Headers["idempotency-key"][0]
            );
            Assert.Equal(1, stub.LogEntries.Count);
        }

        [Fact]
        public void ClientProvidedIdempotencyKeyIsNotOverridden()
        {
            stub.Given(Request.Create().WithPath("/api/v1/app"))
                .RespondWith(Response.Create().WithStatusCode(200).WithBody(applicationOutJsonStr));

            var options = new ApplicationCreateOptions { IdempotencyKey = "test-key-123" };

            client.Application.Create(new ApplicationIn { Name = "app" }, options);

            Assert.Equal(1, stub.LogEntries.Count);
            Assert.True(stub.LogEntries[0].RequestMessage.Headers.ContainsKey("idempotency-key"));
            Assert.Equal(
                "test-key-123",
                stub.LogEntries[0].RequestMessage.Headers["idempotency-key"][0]
            );
            Assert.Equal(1, stub.LogEntries.Count);
        }
    }
}
