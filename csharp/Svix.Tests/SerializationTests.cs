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

        public SerializationTests()
        {
            var port = new Random().Next(5000, 6000);
            baseUrl = "http://localhost:" + port;

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

            Console.WriteLine(baseUrl);
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
    }
}
