using System;
using System.Net;
using Moq;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Xunit;

namespace Svix.Tests
{
    public sealed class HealthTests : IDisposable
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IHealthApi> _mockHealthApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public HealthTests()
        {
            _mockHealthApi = new Mock<IHealthApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                healthApi: _mockHealthApi.Object);
        }

        public void Dispose()
        {
            // empty
        }

        [Fact]
        public void IsHealthy_WhenNotHealthy_ReturnsFalse()
        {
            // Arrange
            _mockHealthApi.Setup(x => x.HealthApiV1HealthGetWithHttpInfo(It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.UnprocessableEntity, null));

            // Act
            var lResult = _svixClient.Health
                .IsHealthy();

            // Assert
            Assert.False(lResult);
        }

        [Fact]
        public void IsHealthy_WhenHealthy_ReturnsTrue()
        {
            // Arrange
            _mockHealthApi.Setup(x => x.HealthApiV1HealthGetWithHttpInfo(It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Health
                .IsHealthy();

            // Assert
            Assert.True(lResult);
        }

        [Fact]
        public async void IsHealthyAsync_WhenNotHealthy_ReturnsFalse()
        {
            // Arrange
            _mockHealthApi.Setup(x => x.HealthApiV1HealthGetWithHttpInfoAsync(It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.UnprocessableEntity, null));

            // Act
            var lResult = await _svixClient.Health
                .IsHealthyAsync();

            // Assert
            Assert.False(lResult);
        }

        [Fact]
        public async void IsHealthyAsync_WhenHealthy_ReturnsTrue()
        {
            // Arrange
            _mockHealthApi.Setup(x => x.HealthApiV1HealthGetWithHttpInfoAsync(It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = await _svixClient.Health
                .IsHealthyAsync();

            // Assert
            Assert.True(lResult);
        }
    }
}