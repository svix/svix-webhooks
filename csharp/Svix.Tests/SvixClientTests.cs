using Microsoft.Extensions.Logging.Abstractions;
using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public class SvixClientTests
    {
        [Fact]
        public void Constructor_WhenCalled_DoesNotNeedLogger()
        {
            var sut = new SvixClient("", new SvixOptions("http://some.url"));

            Assert.NotNull(sut);
        }

        [Fact]
        public void Constructor_WhenCalled_AcceptsLogger()
        {
            var sut = new SvixClient("", new SvixOptions("http://some.url"), new NullLogger<SvixClient>());

            Assert.NotNull(sut);
        }
    }
}