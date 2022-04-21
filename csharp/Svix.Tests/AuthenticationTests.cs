using System;
using System.Net;
using Moq;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Xunit;

namespace Svix.Tests
{
    public sealed class AuthenticationTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IAuthenticationApi> _mockAuthenticationApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public AuthenticationTests()
        {
            _mockAuthenticationApi = new Mock<IAuthenticationApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                authenticationApi: _mockAuthenticationApi.Object);
        }

        [Fact]
        public void AuthenticationGetDashboardAccess_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Authentication.GetDashboardAccess(lApplicationId, lIdempotencyKey);

            // Assert
            _mockAuthenticationApi.Verify(x => x.GetDashboardAccessApiV1AuthDashboardAccessAppIdPost(lApplicationId, lIdempotencyKey));
        }

        [Fact]
        public void AuthenticationGetDashboardAccessAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Authentication.GetDashboardAccessAsync(lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockAuthenticationApi.Verify(x => x.GetDashboardAccessApiV1AuthDashboardAccessAppIdPostAsync(lApplicationId, lIdempotencyKey, default));
        }

        [Fact]
        public void AuthenticationLogout_CallsApi_WithParams()
        {
            // Arrange
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockAuthenticationApi.Setup(x => x.LogoutApiV1AuthLogoutPostWithHttpInfo(It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Authentication.Logout(lIdempotencyKey);

            // Assert
            _mockAuthenticationApi.Verify(x => x.LogoutApiV1AuthLogoutPostWithHttpInfo(lIdempotencyKey));
        }

        [Fact]
        public void AuthenticationLogoutAsync_CallsApi_WithParams()
        {
            // Arrange
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockAuthenticationApi.Setup(x => x.LogoutApiV1AuthLogoutPostWithHttpInfoAsync(It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Authentication.LogoutAsync(lIdempotencyKey, default);

            // Assert
            _mockAuthenticationApi.Verify(x => x.LogoutApiV1AuthLogoutPostWithHttpInfoAsync(lIdempotencyKey, default));
        }
    }
}