using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public sealed class HealthTests
    {
        private const string TOKEN = "";
        
        private static SvixOptions _options = new SvixOptions
        {
            ServerUrl = "https://api.us.svix.com"
        };

        [Fact]
        public void TestIsHealthyWorks()
        {
            // Arrange
            var lSvixClient = new SvixClient(TOKEN, _options);
            
            // Act
            var lIsHealthy = lSvixClient.Health
                .IsHealthy();

            // Assert
            Assert.True(lIsHealthy);
        }
        
        [Fact]
        public async void TestIsHealthyAsyncWorks()
        {
            // Arrange
            var lSvixClient = new SvixClient(TOKEN, _options);
            
            // Act
            var lIsHealthy = await lSvixClient.Health
                .IsHealthyAsync()
                .ConfigureAwait(false);

            // Assert
            Assert.True(lIsHealthy);
        }
    }
}