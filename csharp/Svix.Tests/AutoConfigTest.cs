using System;
using System.Text;
using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public class AutoConfigTests
    {
        private const string AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_";

        [Fact]
        public void ValidToken_DoesNotThrow()
        {
            var json =
                """{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}""";
            var token =
                AUTOCONFIG_TOKEN_PREFIX_V1 + Convert.ToBase64String(Encoding.UTF8.GetBytes(json));

            var ex = Record.Exception(() =>
                new AutoConfig(token, new EndpointIn { Url = "https://hook.example.test" })
            );

            Assert.Null(ex);
        }

        [Fact]
        public void BadPrefix_ThrowsAutoConfigException()
        {
            var json = """{"aid":"a","eid":"e","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}""";
            var token = "wrong_" + Convert.ToBase64String(Encoding.UTF8.GetBytes(json));

            Assert.Throws<AutoConfigException>(() =>
                new AutoConfig(token, new EndpointIn { Url = "https://hook.example.test" })
            );
        }

        [Fact]
        public void InvalidJson_ThrowsAutoConfigException()
        {
            var token =
                AUTOCONFIG_TOKEN_PREFIX_V1
                + Convert.ToBase64String(Encoding.UTF8.GetBytes("not json"));

            Assert.Throws<AutoConfigException>(() =>
                new AutoConfig(token, new EndpointIn { Url = "https://hook.example.test" })
            );
        }
    }
}
