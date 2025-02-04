using System.Collections.Generic;
using Newtonsoft.Json;
using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public class SerializationTests
    {
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
            Assert.Equal(encoded_json, expected_json);
        }
    }
}
