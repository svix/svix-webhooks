using System;
using System.Net;
using Moq;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;
using Xunit;

namespace Svix.Tests
{
    public sealed class IntegrationTests
    {
        private const string MOCK_TOKEN = ";iuani;ansd;ifgjbnai;sdjfgb";

        private readonly Mock<IIntegrationApi> _mockIntegrationApi;

        private readonly Mock<ISvixOptions> _mockOptions;

        private readonly SvixClient _svixClient;

        public IntegrationTests()
        {
            _mockIntegrationApi = new Mock<IIntegrationApi>();
            _mockOptions = new Mock<ISvixOptions>();
            _svixClient = new SvixClient(
                MOCK_TOKEN,
                _mockOptions.Object,
                integrationApi: _mockIntegrationApi.Object);
        }

        [Fact]
        public void IntegrationCreate_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lIntegration = new IntegrationIn("MyIntegration");

            // Act
            var lResult = _svixClient.Integration.Create(lApplicationId, lIntegration, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.CreateIntegrationApiV1AppAppIdIntegrationPost(lApplicationId, lIntegration, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationCreateAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lIntegration = new IntegrationIn("integ_1srOrx2ZWZBpBUvZwXKQmoEYga2");

            // Act
            var lResult = _svixClient.Integration.CreateAsync(lApplicationId, lIntegration, lIdempotencyKey, default);

            // Assert
            _mockIntegrationApi.Verify(x => x.CreateIntegrationApiV1AppAppIdIntegrationPostAsync(lApplicationId, lIntegration, lIdempotencyKey, default));
        }

        [Fact]
        public void IntegrationDelete_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockIntegrationApi.Setup(x => x.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteWithHttpInfo(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Integration.Delete(lIntegration, lApplicationId, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteWithHttpInfo(lApplicationId, lIntegration, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationDeleteAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockIntegrationApi.Setup(x => x.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteWithHttpInfoAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new ApiResponse<object>(HttpStatusCode.NoContent, null));

            // Act
            var lResult = _svixClient.Integration.DeleteAsync(lIntegration, lApplicationId, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteWithHttpInfoAsync(lApplicationId, lIntegration, lIdempotencyKey, default));
        }

        [Fact]
        public void IntegrationGet_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Integration.Get(lIntegration, lApplicationId, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.GetIntegrationApiV1AppAppIdIntegrationIntegIdGet(lApplicationId, lIntegration, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationGetAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            // Act
            var lResult = _svixClient.Integration.GetAsync(lIntegration, lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockIntegrationApi.Verify(x => x.GetIntegrationApiV1AppAppIdIntegrationIntegIdGetAsync(lApplicationId, lIntegration, lIdempotencyKey, default));
        }

        [Fact]
        public void IntegrationGetKey_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockIntegrationApi.Setup(x => x.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new IntegrationKeyOut(Guid.NewGuid().ToString()));

            // Act
            var lResult = _svixClient.Integration.GetKey(lIntegration, lApplicationId, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(lApplicationId, lIntegration, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationGetKeyAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockIntegrationApi.Setup(x => x.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGetAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new IntegrationKeyOut(Guid.NewGuid().ToString()));

            // Act
            var lResult = _svixClient.Integration.GetKeyAsync(lIntegration, lApplicationId, lIdempotencyKey, default);

            // Assert
            _mockIntegrationApi.Verify(x => x.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGetAsync(lApplicationId, lIntegration, lIdempotencyKey, default));
        }

        [Fact]
        public void IntegrationList_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new ListOptions
            {
                Iterator = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Limit = 50
            };

            // Act
            var lResult = _svixClient.Integration.List(lApplicationId, lOptions, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.ListIntegrationsApiV1AppAppIdIntegrationGet(lApplicationId, lOptions.Iterator, lOptions.Limit, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationListAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lOptions = new ListOptions
            {
                Iterator = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2",
                Limit = 50
            };

            // Act
            var lResult = _svixClient.Integration.ListAsync(lApplicationId, lOptions, lIdempotencyKey, default);

            // Assert
            _mockIntegrationApi.Verify(x => x.ListIntegrationsApiV1AppAppIdIntegrationGetAsync(lApplicationId, lOptions.Iterator, lOptions.Limit, lIdempotencyKey, default));
        }

        [Fact]
        public void IntegrationRotateKey_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockIntegrationApi.Setup(x => x.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>()))
                .Returns(new IntegrationKeyOut(Guid.NewGuid().ToString()));

            // Act
            var lResult = _svixClient.Integration.RotateKey(lApplicationId, lIntegration, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(lIntegration, lApplicationId, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationRotateKeyAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegration = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            _mockIntegrationApi.Setup(x => x.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePostAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), default))
                .ReturnsAsync(new IntegrationKeyOut(Guid.NewGuid().ToString()));

            // Act
            var lResult = _svixClient.Integration.RotateKeyAsync(lApplicationId, lIntegration, lIdempotencyKey, default);

            // Assert
            _mockIntegrationApi.Verify(x => x.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePostAsync(lIntegration, lApplicationId, lIdempotencyKey, default));
        }

        [Fact]
        public void IntegrationUpdate_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegrationId = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lIntegration = new IntegrationUpdate("Updated Name");

            // Act
            var lResult = _svixClient.Integration.Update(lApplicationId, lIntegrationId, lIntegration, lIdempotencyKey);

            // Assert
            _mockIntegrationApi.Verify(x => x.UpdateIntegrationApiV1AppAppIdIntegrationIntegIdPut(lIntegrationId, lApplicationId, lIntegration, lIdempotencyKey));
        }

        [Fact]
        public void IntegrationUpdateAsync_CallsApi_WithParams()
        {
            // Arrange
            string lApplicationId = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIntegrationId = "integ_1srOrx2ZWZBpBUvZwXKQmoEYga2";
            string lIdempotencyKey = Guid.NewGuid().ToString();

            var lIntegration = new IntegrationUpdate("Updated Name");

            // Act
            var lResult = _svixClient.Integration.UpdateAsync(lApplicationId, lIntegrationId, lIntegration, lIdempotencyKey, default);

            // Assert
            _mockIntegrationApi.Verify(x => x.UpdateIntegrationApiV1AppAppIdIntegrationIntegIdPutAsync(lIntegrationId, lApplicationId, lIntegration, lIdempotencyKey, default));
        }
    }
}