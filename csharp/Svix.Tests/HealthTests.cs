using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public sealed class HealthTests
    {
        private static SvixClientOptions ClientOptions = new SvixClientOptions
        {
            AccessToken = "",
            ServiceUrl = ""
        };
        
        [Fact]
        public void TestIsHealthyWorks()
        {
            // Arrange
            var lSvixClient = new SvixClient(ClientOptions);
            
            // Act
            var lIsHealthy = lSvixClient.Health.IsHealthy();

            // Assert
            Assert.True(lIsHealthy);
        }
        
        [Fact]
        public async void TestIsHealthyAsyncWorks()
        {
            // Arrange
            var lSvixClient = new SvixClient(ClientOptions);
            
            // Act
            var lIsHealthy = await lSvixClient.Health.IsHealthyAsync()
                .ConfigureAwait(false);

            // Assert
            Assert.True(lIsHealthy);
        }
    }
}